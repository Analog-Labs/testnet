use std::{fs::File, io::Read, str::FromStr};
use tokio::sync::mpsc;
use web3::{
	contract::{Contract, Options},
	transports::Http,
	types::Address,
}; // U256

#[derive(Clone)]
pub struct SwapToken {
	pub web_socket: web3::Web3<Http>,
	pub sender: mpsc::Sender<String>,
}

impl SwapToken {
	/// create a new event instance
	pub fn new(web_socket: web3::Web3<Http>, sender: mpsc::Sender<String>) -> Self {
		SwapToken { web_socket, sender }
	}
	/// This function is to fetch the swap price of a token.
	pub async fn swap_price<P: web3::contract::tokens::Tokenize>(
		web_socket: &web3::Web3<Http>,
		abi_url: &str,
		exchange_address: &str,
		query_method: &str,
		_query_parameter: P,
	) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
		let exchange = match Address::from_str(exchange_address) {
			Ok(exchange) => exchange,
			Err(error) => return Err(From::from(error)),
		};
		let mut res = String::new();
		if abi_url.contains("http") {
			if let Ok(response) = reqwest::blocking::get(abi_url) {
				if let Ok(text) = response.text() {
					res = text;
				} else {
					return Err("Failed to get response text".into());
				}
			} else {
				return Err("Failed to make GET request".into());
			}
		} else {
			let mut abi_file = match File::open(abi_url) {
				Ok(file) => file,
				Err(error) => return Err(From::from(error)),
			};
			match abi_file.read_to_string(&mut res) {
				Ok(_) => (),
				Err(error) => return Err(From::from(error)),
			};
		}

		let json: serde_json::Value =
			serde_json::from_str(&res.to_owned()).expect("JSON was not well-formatted");

		let abi_date = match serde_json::to_string(&json["abi"]) {
			Ok(response_str) => response_str,
			Err(_) => "data Error".to_string(),
		};
		let token_contract =
			match Contract::from_json(web_socket.eth(), exchange, abi_date.as_bytes()) {
				Ok(contract) => contract,
				Err(error) => return Err(From::from(error)),
			};
		let query_response: Vec<u8> = match token_contract
			.query(query_method, (exchange, exchange, 25), None, Options::default(), None)
			.await
		{
			Ok(query_response) => query_response,
			Err(error) => return Err(From::from(error)),
		};
		Ok(query_response)
	}

	/// This function is to fetch the decimals digit of a token.
	pub async fn decimals(
		web_socket: &web3::Web3<Http>,
		token_abi_url: &str,
		token_address: &str,
	) -> Result<i32, Box<dyn std::error::Error>> {
		let exchange = match Address::from_str(token_address) {
			Ok(address) => address,
			Err(error) => return Err(From::from(error)),
		};
		let mut abi_file = match File::open(token_abi_url) {
			Ok(file) => file,
			Err(error) => return Err(From::from(error)),
		};
		let mut res = String::new();
		match abi_file.read_to_string(&mut res) {
			Ok(_) => (),
			Err(error) => return Err(From::from(error)),
		};
		let json: serde_json::Value =
			serde_json::from_str(&res.to_owned()).expect("JSON was not well-formatted");

		let abi_date = match serde_json::to_string(&json["abi"]) {
			Ok(response_str) => response_str,
			Err(_) => "data Error".to_string(),
		};

		// Accessing existing contract of exchange.
		let token_contract =
			match Contract::from_json(web_socket.eth(), exchange, abi_date.as_bytes()) {
				Ok(contract) => contract,
				Err(error) => return Err(From::from(error)),
			};
		// fetching the decimal of a particular token.
		token_contract
			.query("decimals", (), None, Options::default(), None)
			.await
			.map_err(Into::into)
	}
	pub async fn decimals_old(
		web_socket: &web3::Web3<Http>,
		token_abi_url: &str,
		token_address: &str,
	) -> Result<i32, Box<dyn std::error::Error>> {
		let exchange = match Address::from_str(token_address) {
			Ok(address) => address,
			Err(error) => return Err(From::from(error)),
		};
		// todo file or url
		let res = match reqwest::blocking::get(token_abi_url) {
			Ok(url) => match url.text() {
				Ok(url) => url,
				Err(error) => return Err(From::from(error)),
			},
			Err(error) => return Err(From::from(error)),
		};

		let json: serde_json::Value =
			serde_json::from_str(&res.to_owned()).expect("JSON was not well-formatted");
		log::info!("atomic values ----4");
		let abi: String = match &json["result"] {
			serde_json::Value::String(v) => v.clone(),
			_ => String::from(""),
		};

		// Accessing existing contract of exchange.
		let token_contract = match Contract::from_json(web_socket.eth(), exchange, abi.as_bytes()) {
			Ok(contract) => contract,
			Err(error) => return Err(From::from(error)),
		};
		// fetching the decimal of a particular token.
		token_contract
			.query("decimals", (), None, Options::default(), None)
			.await
			.map_err(Into::into)
	}
}
