use futures::future::Future;
use serde::{Serialize, Deserialize};

use crate::client::Client;
use crate::error::Error;

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
    /// Refund fee in satoshis
    pub fee: Option<u64>,
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
    Pending
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
        Payload{
            checkout_id: id.into(),
            address: address.into(),
            email: None,
        }
    }
}

/// Create refund
pub fn create(client: &Client, payload: Payload) -> impl Future<Item=Refund, Error=Error> {
    client.post("/v1/refunds", Some(payload))
}

/// Retrieve refund with the given id
pub fn get(client: &Client, id: &str) -> impl Future<Item=Refund, Error=Error> {
    client.get(format!("/v1/refund/{}", id), None as Option<String>)
}

/// Retrieve refunds.
pub fn list(client: &Client) -> impl Future<Item=Vec<Refund>, Error=Error> {
    client.get("/v1/refunds", None as Option<String>)
}
