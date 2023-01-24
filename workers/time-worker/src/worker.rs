use crate::{
	communication::validator::{topic, GossipValidator},
	inherents::update_shared_group_key,
	kv::TimeKeyvault,
	Client, WorkerParams, TW_LOG,
};
use borsh::{BorshDeserialize, BorshSerialize};
use futures::{channel::mpsc::Receiver as FutReceiver, FutureExt, StreamExt};
use log::{debug, error, info, warn};
use sc_client_api::{Backend, FinalityNotification, FinalityNotifications};
use sc_network_gossip::GossipEngine;
use sp_api::ProvideRuntimeApi;
use sp_consensus::SyncOracle;
use sp_runtime::traits::{Block, Header};
use std::{sync::Arc, time::Duration};
use time_primitives::{TimeApi, KEY_TYPE};
use tokio::sync::Mutex as TokioMutex;
use tss::{
	frost_dalek::{compute_message_hash, SignatureAggregator},
	local_state_struct::TSSLocalStateData,
	tss_event_model::{PartialMessageSign, TSSData, TSSEventType},
	utils::{get_receive_params_msg, make_gossip_tss_data},
};

#[allow(unused)]
/// Our structure, which holds refs to everything we need to operate
pub struct TimeWorker<B: Block, C, R, BE, SO> {
	client: Arc<C>,
	backend: Arc<BE>,
	runtime: Arc<R>,
	finality_notifications: FinalityNotifications<B>,
	gossip_engine: GossipEngine<B>,
	gossip_validator: Arc<GossipValidator<B>>,
	sign_data_receiver: Arc<TokioMutex<FutReceiver<(u64, Vec<u8>)>>>,
	pub(crate) kv: TimeKeyvault,
	sync_oracle: SO,
	pub(crate) tss_local_state: TSSLocalStateData,
}

impl<B, C, R, BE, SO> TimeWorker<B, C, R, BE, SO>
where
	B: Block,
	BE: Backend<B>,
	C: Client<B, BE>,
	R: ProvideRuntimeApi<B>,
	R::Api: TimeApi<B>,
	SO: SyncOracle + Send + Sync + Clone + 'static,
{
	pub(crate) fn new(worker_params: WorkerParams<B, C, R, BE, SO>) -> Self {
		let WorkerParams {
			client,
			backend,
			runtime,
			gossip_engine,
			gossip_validator,
			sync_oracle,
			kv,
			sign_data_receiver,
		} = worker_params;

		let mut tss_local_state = TSSLocalStateData::new();
		tss_local_state.key_type = Some(KEY_TYPE);
		tss_local_state.keystore = kv.get_store();
		// TODO: threshold calc if required

		TimeWorker {
			finality_notifications: client.finality_notification_stream(),
			client,
			backend,
			runtime,
			gossip_engine,
			gossip_validator,
			sync_oracle,
			kv,
			tss_local_state,
			sign_data_receiver,
		}
	}

	/// On each grandpa finality we're initiating gossip to all other authorities to acknowledge
	fn on_finality(&mut self, notification: FinalityNotification<B>) {
		info!(target: TW_LOG, "Got new finality notification: {}", notification.header.number());
		let _number = notification.header.number();
		let keys = self.kv.public_keys();
		if keys.is_empty() {
			warn!(target: TW_LOG, "No time key found, please inject one.");
			return;
		}
		let id = keys[0].clone();
		// starting TSS initialization if not yet done

		if self.tss_local_state.local_peer_id.is_none() {
			self.tss_local_state.local_peer_id = Some(id.to_string());
			// TODO: reset if not all comply
			// slashing goes below too?
			/*			let msg = make_gossip_tss_data(
							id.to_string(),
							get_reset_tss_msg("Reinit state".to_string()).unwrap_or_default(),
							TSSEventType::ReceiveParams,
						)
						.unwrap();
						self.send(msg);
			*/
			// initialize TSS
			// TODO: assign collector role to at most one node for protocol to work
			let collector_id_bytes =
				hex::decode("48640c12bc1b351cf4b051ac1cf7b5740765d02e34989d0a9dd935ce054ebb21")
					.unwrap();
			let collector_id = sp_application_crypto::sr25519::Public::from_raw(
				array_bytes::vec2array(collector_id_bytes).unwrap(),
			);
			let mut for_tests = false;
			// we allow alice to be collector and aggregator in tests
			#[cfg(test)]
			if id == crate::tests::kv_tests::Keyring::Alice.public() {
				for_tests = true;
			}
			if id == collector_id.into() || for_tests {
				self.tss_local_state.is_node_collector = true;
				// TODO: assign aggregator role to at most one node for protocol to work
				self.tss_local_state.is_node_aggregator = true;
				let local_peer_id = self.tss_local_state.local_peer_id.clone().unwrap();
				if let Ok(peer_id_data) =
					get_receive_params_msg(local_peer_id.clone(), self.tss_local_state.tss_params)
				{
					if let Ok(data) = make_gossip_tss_data(
						local_peer_id,
						peer_id_data,
						TSSEventType::ReceiveParams,
					) {
						self.send(data);
						info!("TSS peer collection req sent");
					}
				} else {
					log::error!("Unable to make publish peer id msg");
				}
			}

			info!(target: TW_LOG, "Gossip is sent.");
		}
	}

	pub(crate) fn send(&mut self, msg: Vec<u8>) {
		info!(target: TW_LOG, "Sending new gossip message");
		self.gossip_engine.gossip_message(topic::<B>(), msg, false);
	}

	#[allow(dead_code)]
	// Using this method each validator can, and should, submit shared `GroupKey` key to runtime
	pub(crate) fn submit_key_as_inherent(&self, key: [u8; 32], set_id: u64) {
		update_shared_group_key(set_id, key);
	}

	/// On each gossip we process it, verify signature and update our tracker
	async fn on_gossip(&mut self, tss_gossiped_data: TSSData) {
		info!(target: TW_LOG, "Processing new gossip");
		match tss_gossiped_data.tss_event_type {
			//nodes will be receiving this event to make participant using params
			TSSEventType::ReceiveParams => {
				self.handler_receive_params(&tss_gossiped_data.tss_data).await;
			},
			// nodes will receive peer id of other nodes and will add it to their list
			TSSEventType::ReceivePeerIDForIndex => {
				self.handler_receive_peer_id_for_index(&tss_gossiped_data.tss_data).await;
			}, //nodes receives peers with collector participants
			TSSEventType::ReceivePeersWithColParticipant => {
				self.handler_receiver_peers_with_col_participant(&tss_gossiped_data.tss_data)
					.await;
			},
			//nodes will receive participant and will add will go to round one state
			TSSEventType::ReceiveParticipant => {
				self.handler_receive_participant(&tss_gossiped_data.tss_data).await;
			},
			//nodes will receive their secret share and take state to round two
			TSSEventType::ReceiveSecretShare => {
				info!(target: TW_LOG, "Received ReceiveSecretShare");
				self.handler_receive_secret_share(&tss_gossiped_data.tss_data).await;
			},

			//received commitments of other nodes who are participating in TSS process
			TSSEventType::ReceiveCommitment => {
				self.handler_receive_commitment(&tss_gossiped_data.tss_data).await;
			},

			//event received by collector and partial sign request is received
			TSSEventType::PartialSignatureGenerateReq => {
				self.handler_partial_signature_generate_req(&tss_gossiped_data.tss_data).await;
			},

			//received partial signature. threshold signature to be made by aggregator
			TSSEventType::PartialSignatureReceived => {
				self.handler_partial_signature_received(&tss_gossiped_data.tss_data).await;
			},

			//verify threshold signature generated by aggregator
			TSSEventType::VerifyThresholdSignature => {
				self.handler_verify_threshold_signature(&tss_gossiped_data.tss_data).await;
			},

			//received resetting tss state request
			TSSEventType::ResetTSSState => {
				self.handler_reset_tss_state(&tss_gossiped_data.tss_data).await;
			},
		}
	}

	/// Our main worker main process - we act on grandpa finality and gossip messages for interested
	/// topics
	pub(crate) async fn run(&mut self) {
		let mut gossips = Box::pin(
			self.gossip_engine
				.messages_for(topic::<B>())
				.filter_map(|notification| async move {
					debug!(target: TW_LOG, "Got new gossip message");
					TSSData::try_from_slice(&notification.message).ok()
				})
				.fuse(),
		);
		loop {
			let receiver = self.sign_data_receiver.clone();
			let mut signature_requests = receiver.lock().await;
			// making sure majority is synchronising or waiting for the sync
			while self.sync_oracle.is_major_syncing() {
				debug!(target: TW_LOG, "Waiting for major sync to complete...");
				futures_timer::Delay::new(Duration::from_secs(1)).await;
			}

			let mut engine = &mut self.gossip_engine;

			futures::select_biased! {
				_ = engine => {
					error!(
						target: TW_LOG,
						"Gossip engine has terminated."
					);
					return;
				},
				notification = self.finality_notifications.next().fuse() => {
					if let Some(notification) = notification {
						self.on_finality(notification);
					} else {
						debug!(
							target: TW_LOG,
							"no new finality notifications"
						);
					}
				},
				new_sig = signature_requests.next().fuse() => {
					if let Some((group_id, data)) = new_sig {
						// do sig
						let context = self.tss_local_state.context;
						let msg_hash = compute_message_hash(&context, &data);
						//add node in msg_pool
						if !self.tss_local_state.msg_pool.contains_key(&msg_hash){
							self.tss_local_state.msg_pool.insert(msg_hash, data.clone().into());
							//process msg if req already received
							if let Some(pending_msg_req) = self.tss_local_state.msgs_signature_pending.get(&msg_hash) {
								let request = PartialMessageSign {
									msg_hash,
									signers: pending_msg_req.to_vec()
								};
								let encoded = request.try_to_vec().unwrap();
								self.handler_partial_signature_generate_req(&encoded).await;
								self.tss_local_state.msgs_signature_pending.remove(&msg_hash);
							} else {
								log::debug!(
									target: TW_LOG,
									"New data for signing received: {:?} with hash {:?}",
									data,
									msg_hash
								);
							}
						} else {
							log::warn!(
								target: TW_LOG,
								"Message with hash {} is already in process of signing",
								hex::encode(&msg_hash)
							);
						}
						//creating signature aggregator for msg
						if self.tss_local_state.is_node_aggregator {
							//all nodes should share the same message hash
							//to verify the threshold signature
							let mut aggregator = SignatureAggregator::new(
								self.tss_local_state.tss_params,
								self.tss_local_state.local_finished_state.clone().unwrap().0,
								&context,
								&data,
							);

							for com in self.tss_local_state.others_commitment_share.clone(){
								aggregator.include_signer(
									com.public_commitment_share_list.participant_index,
									com.public_commitment_share_list.commitments[0],
									com.public_key,
								);
							}

							//including aggregator as a signer
							aggregator.include_signer(
								self.tss_local_state.local_index.clone().unwrap(),
								self.tss_local_state.local_commitment_share.clone().unwrap().0.commitments[0],
								self.tss_local_state.local_public_key.clone().unwrap(),
							);

							//this signers list will be used by other nodes to verify themselves.
							let signers = aggregator.get_signers();
							self.tss_local_state.current_signers = signers.clone();

							// //sign msg from aggregator side
							self.aggregator_event_sign(msg_hash.clone()).await;

							let sign_msg_req = PartialMessageSign{
								msg_hash,
								signers: signers.clone(),
							};

							if let Ok(data) = make_gossip_tss_data(
								self.tss_local_state.local_peer_id.clone().unwrap_or_default(),
								sign_msg_req.try_to_vec().unwrap(),
								TSSEventType::PartialSignatureGenerateReq,
							) {
								self.send(data);
								info!( target: TW_LOG, "TSS peer collection req sent");
							} else {
								error!(target: TW_LOG, "Failed to pack TSS message for signing hash: {}", hex::encode(&msg_hash));
							}
						}
					}
				},
				gossip = gossips.next() => {
					if let Some(gossip) = gossip {
						self.on_gossip(gossip).await;
					} else {
						debug!(
							target: TW_LOG,
							"no new gossips"
						);
					}
				},
			}
		}
	}
}
