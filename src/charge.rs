use futures::future::Future;
use serde::{Serialize, Deserialize};

use crate::invoice;
use crate::client::Client;
use crate::error::Error;

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
	pub status: String,
	/// Charge fee in satoshis
	pub fee: u64,
	/// Charge value at issue time
	pub fiat_value: f64,
	/// Charge currency
	pub currency: String,
	/// timestamp
	pub created_at: u64,
	/// URL where user gets redirected after payment
	pub success_url: String,
	/// Charge notes
	pub notes: String,
	/// Charge requested instant exchange
	pub auto_settle: bool,
	/// External order ID
	pub order_id: String,
	/// Charge payment details
	pub chain_invoice: invoice::Chain,
	/// lightning_invoice
	pub lightning_invoice: invoice::Lightning,
	/// Hashed Order
	/// OpenNode signs all charge related events it sends
	/// to your endpoints by including a hashed_order field
	/// on the event payload. This allows you to validate that
	/// the events were sent by OpenNode and not by a third party.
	pub hashed_order: String,
}


/// Payload is a charge payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    /// Charge's price in satoshis, unless currency parameter is used.
    pub amount: u64,
    /// Charge's description
    pub description: Option<String>,
    /// Charge's currency
    pub currency: Option<String>,
    /// Customer's email
    pub customer_email: Option<String>,
    /// Customer's name
    pub customer_name: Option<String>,
    /// URL to receive webhooks
    pub callback_url: Option<String>,
    /// URL to redirect user after payment
    pub success_url: Option<String>,
    /// Convert to merchant's currency (needs Bank account enabled)
    pub auto_settle: Option<String>
}

impl Payload {
    pub fn new(a: u64) -> Payload {
        Payload{
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
pub fn create(client: &Client, payload: Payload) -> impl Future<Item=Charge, Error=Error> {
    client.post("/charges", Some(payload))
}

/// Retrieve charge with the given id
pub fn get(client: &Client, id: u64) -> impl Future<Item=Charge, Error=Error> {
    client.get(format!("/charges/{}", id), None as Option<String>)
}

