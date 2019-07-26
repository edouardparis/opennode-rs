use futures::future::Future;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::currency::Currency;
use crate::error::Error;
use crate::invoice;

/// Charge is a charge resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Charge {
    pub id: String,
    /// Charge Description
    pub description: String,
    /// Charge price in satoshis
    pub amount: u64,
    /// Charge status
    /// unpaid/processing/paid
    pub status: Status,
    /// Charge fee in satoshis
    pub fee: Option<u64>,
    /// Charge value at issue time
    pub fiat_value: f64,
    /// Charge currency
    pub currency: Option<Currency>,
    /// timestamp
    pub created_at: u64,
    /// URL where user gets redirected after payment
    pub success_url: Option<String>,
    /// Charge notes
    pub notes: String,
    /// Charge requested instant exchange
    pub auto_settle: bool,
    /// External order ID
    pub order_id: Option<String>,
    /// Charge payment details
    pub chain_invoice: Option<invoice::Chain>,
    /// lightning_invoice
    pub lightning_invoice: Option<invoice::Lightning>,
    /// Hashed Order
    /// OpenNode signs all charge related events it sends
    /// to your endpoints by including a hashed_order field
    /// on the event payload. This allows you to validate that
    /// the events were sent by OpenNode and not by a third party.
    pub hashed_order: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unpaid")]
    Unpaid,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "underpaid")]
    Underpaid,
    #[serde(rename = "refunded")]
    Refunded,
}

/// Payload is a charge payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    /// Charge's price in satoshis, unless currency parameter is used.
    pub amount: u64,
    /// Charge's description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Charge's currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Customer's email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    /// Customer's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// URL to receive webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// URL to redirect user after payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    /// Convert to merchant's currency (needs Bank account enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_settle: Option<String>,
}

impl Payload {
    pub fn new(a: u64) -> Payload {
        Payload {
            amount: a,
            description: None,
            currency: None,
            customer_email: None,
            customer_name: None,
            callback_url: None,
            success_url: None,
            auto_settle: None,
        }
    }
}

/// Create charge
pub fn create(client: &Client, payload: Payload) -> impl Future<Item = Charge, Error = Error> {
    client.post("/v1/charges", Some(payload))
}

/// Retrieve charge with the given id
pub fn get(client: &Client, id: &str) -> impl Future<Item = Charge, Error = Error> {
    client.get(format!("/v1/charge/{}", id), None as Option<String>)
}

/// Retrieve paid charges.
pub fn list(client: &Client) -> impl Future<Item = Vec<Charge>, Error = Error> {
    client.get("/v1/charges", None as Option<String>)
}
