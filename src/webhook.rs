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
use crate::charge::Status;
use crate::charge::Withdrawal;

/// Charge is a event resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Charge {
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
    ///
    /// You can verify the signatures by computing an HMAC with the SHA256 hash function. Use the
    /// api-key used on the charge creation as the key, and the charge id as the message.
    pub hashed_order: Option<String>,
}

/// When initiating a Lightning Network withdrawal, you can register a webhook URL
/// (via callback_url parameter). The webhook URL will notify you if your payment succeeded
/// or failed to reach its destination.
/// Withdrawal is a the webhook resource.
type struct Withdrawal = Withdrawal;
