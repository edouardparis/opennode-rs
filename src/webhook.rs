/// OpenNode sends webhook events to notify your application any time there is an update to your
/// charges or withdrawals.
///
/// For example, you can specify a callback_url parameter when creating a charge and OpenNode will
/// send you webhook events for any update regarding that charge - e.g. the charge was successfully
/// paid.
///
/// OpenNode offers a webhook simulator tool to helps developers test their webhook handling. The
/// tool will simulate a parameter check without having to make a real transaction.

use serde::{Serialize, Deserialize};

use crate::refund::Refund;
use crate::transaction::Transaction;

/// Event is a event resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    /// URL where webhooks is sent
    pub callback_url: Option<String>,
    /// URL where user gets redirected after payment
    pub success_url: Option<String>,
    /// Charge status
    /// underpaid/processing/paid/refunded
    pub status: Status,
    /// External order ID
    pub order_id: Option<String>,
    /// Charge Description
    pub description: String,
    /// Charge price in satoshis
    pub price: u64,
    /// Charge fee in satoshis
    pub fee: Option<u64>,
    /// Charge requested instant exchange
    pub auto_settle: bool,
    /// Address
    pub address: Option<String>,
    /// Missing amount
    pub missing_amt: Option<u64>,
    /// transactions are the user onchain transactions
    pub transactions: Option<Vec<Transaction>>,
    /// Refund information
    pub refund: Option<Refund>
    /// Hashed Order
    /// OpenNode signs all charge related events it sends
    /// to your endpoints by including a hashed_order field
    /// on the event payload. This allows you to validate that
    /// the events were sent by OpenNode and not by a third party.
    pub hashed_order: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "refunded")]
    Refunded,
    #[serde(rename = "underpaid")]
    Underpaid,
}

