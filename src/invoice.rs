use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
    ///  Bitcoin address
    pub address: String,
    /// Charge settlement timestamp
    pub settled_at: Option<u64>,
    /// Transaction ID on Bitcoin Blockchain
    pub tx: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lightning {
    /// Payment Request creation timestamp
    pub created_at: Option<u64>,
    /// Charge settlement timestamp
    pub settled_at: Option<u64>,
    /// Payment Request hash
    pub payreq: String,
}
