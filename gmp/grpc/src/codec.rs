use bytes::{Buf, BufMut};
use std::marker::PhantomData;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::Status;

#[derive(Debug)]
pub struct BincodeEncoder<T>(PhantomData<T>);

impl<T: serde::Serialize> Encoder for BincodeEncoder<T> {
	type Item = T;
	type Error = Status;

	fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
		serde_json::to_writer(buf.writer(), &item).map_err(|e| Status::internal(e.to_string()))
	}
}

#[derive(Debug)]
pub struct BincodeDecoder<U>(PhantomData<U>);

impl<U: serde::de::DeserializeOwned> Decoder for BincodeDecoder<U> {
	type Item = U;
	type Error = Status;

	fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
		let item: Self::Item =
			serde_json::from_reader(buf.reader()).map_err(|e| Status::internal(e.to_string()))?;
		Ok(Some(item))
	}
}

/// A [`Codec`] that implements `application/grpc+json` via the serde library.
#[derive(Debug, Clone)]
pub struct BincodeCodec<T, U>(PhantomData<(T, U)>);

impl<T, U> Default for BincodeCodec<T, U> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<T, U> Codec for BincodeCodec<T, U>
where
	T: serde::Serialize + Send + 'static,
	U: serde::de::DeserializeOwned + Send + 'static,
{
	type Encode = T;
	type Decode = U;
	type Encoder = BincodeEncoder<T>;
	type Decoder = BincodeDecoder<U>;

	fn encoder(&mut self) -> Self::Encoder {
		BincodeEncoder(PhantomData)
	}

	fn decoder(&mut self) -> Self::Decoder {
		BincodeDecoder(PhantomData)
	}
}
