use futures::future::Future;
use serde::{Serialize, Deserialize};

use crate::client::Client;
use crate::error::Error;

/// Withdrawal is a withdrawal resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Withdrawal {
    pub id: String,
    /// Withdrawal amount in satoshis
    pub amount: u64,
    /// Withdrawal fiat_value
    pub fiat_value: Option<u64>,
    /// kind of the transaction ln/chain
    #[serde(rename = "type")]
    pub kind: Type,
    /// invoice or tx
    pub reference: String,
    /// Withdrawal fee in satoshis
    pub fee: Option<u64>,
    /// unpaid/processing/paid
    pub status: Status,
    /// timestamp
    pub processed_at: Option<u64>,
    /// error if status == failed
    pub error: Option<String>,
    /// Hashed Order
    /// OpenNode signs all charge related events it sends
    /// to your endpoints by including a hashed_order field
    /// on the event payload. This allows you to validate that
    /// the events were sent by OpenNode and not by a third party.
    pub hashed_order: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ln")]
    Ln,
    #[serde(rename = "chain")]
    Chain
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed
}

/// Payload is a withdrawal payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    /// Withdrawal type (ln/chain)
    #[serde(rename = "type")]
    pub kind: Type,
    /// Bitcoin address/LN BOLT11 invoice
    pub address: String,
    /// Amount to withdraw in satoshis (required when type = 'chain')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// URL to receive withdrawal webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

impl Payload {
    pub fn new(t: Type, address: impl Into<String>, amount: Option<u64>) -> Payload {
        Payload{
            kind: t,
            address: address.into(),
            amount: amount,
            callback_url: None
        }
    }
}

/// Create withdrawal
pub fn create(client: &Client, payload: Payload) -> impl Future<Item=Withdrawal, Error=Error> {
    client.post("/v2/withdrawals", Some(payload))
}

/// Retrieve withdrawal with the given id
pub fn get(client: &Client, id: &str) -> impl Future<Item=Withdrawal, Error=Error> {
    client.get(format!("/v1/withdrawal/{}", id), None as Option<String>)
}

/// Retrieve withdrawals.
pub fn list(client: &Client) -> impl Future<Item=Vec<Withdrawal>, Error=Error> {
    client.get("/v1/withdrawals", None as Option<String>)
}
