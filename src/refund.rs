use serde::{Deserialize, Serialize};

/// Refund is a refund resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Refund {
    pub id: String,
    /// Underpaid charge ID
    pub checkout_id: String,
    /// Bitcoin address to send the funds
    pub address: String,
    /// unpaid/processing/paid
    pub status: Status,
    /// Amount in satoshis
    pub amount: String,
    /// Refund fee in satoshis
    pub fee: Option<String>,
    /// Buyer email to get notified of the refund
    pub email: Option<String>,
    /// timestamp
    pub created_at: u64,
    /// timestamp
    pub processed_at: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "pending")]
    Pending,
}

/// Payload is a refund payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    /// Underpaid charge ID
    pub checkout_id: String,
    /// Bitcoin address to send the funds
    pub address: String,
    /// Refund's email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl Payload {
    pub fn new(id: impl Into<String>, address: impl Into<String>) -> Payload {
        Payload {
            checkout_id: id.into(),
            address: address.into(),
            email: None,
        }
    }
}
