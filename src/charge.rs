use serde::{Serialize, Deserialize};

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



