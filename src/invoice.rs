use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
	///  Bitcoin address
	pub address: String,
	/// Charge settlement timestamp
	pub settled_at: u64,
	/// Transaction ID on Bitcoin Blockchain
	pub tx: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lightning {
	/// Payment Request creation timestamp
	pub created_at: u64,
	/// Charge settlement timestamp
	pub settled_at: u64,
	/// Payment Request hash
	pub payreq: String,
}
