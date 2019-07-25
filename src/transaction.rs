use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    ///  Bitcoin address
    pub address: String,
    /// Transaction creation timestamp
    pub created_at: Option<u64>,
    /// Transaction settlement timestamp
    pub settled_at: Option<u64>,
    /// Transaction ID on Bitcoin Blockchain
    pub tx: Option<String>,
    /// amount in satoshis
    pub amount: u64,
    /// Tx status
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "confirmed")]
    Confirmed,
}
