pub mod client;
pub mod error;

use opennode::account::{Balance};
use opennode::charge;
use opennode::charge::Charge;
use opennode::withdrawal;
use opennode::withdrawal::Withdrawal;
use opennode::refund;
use opennode::refund::Refund;
use opennode::currency::Currency;
use opennode::rate::Rates;

use futures::future::Future;

use crate::client::Client;
use crate::error::Error;

/// Create charge
pub fn create_charge(client: &Client, payload: charge::Payload) -> impl Future<Item = Charge, Error = Error> {
    client.post("/v1/charges", Some(payload))
}

/// Retrieve charge with the given id
pub fn get_charge(client: &Client, id: &str) -> impl Future<Item = Charge, Error = Error> {
    client.get(format!("/v1/charge/{}", id), None as Option<String>)
}

/// Retrieve paid charges.
pub fn list_charges(client: &Client) -> impl Future<Item = Vec<Charge>, Error = Error> {
    client.get("/v1/charges", None as Option<String>)
}

/// Create withdrawal
pub fn create_withdrawal(client: &Client, payload: withdrawal::Payload) -> impl Future<Item = Withdrawal, Error = Error> {
    client.post("/v2/withdrawals", Some(payload))
}

/// Retrieve withdrawal with the given id
pub fn get_withdrawal(client: &Client, id: &str) -> impl Future<Item = Withdrawal, Error = Error> {
    client.get(format!("/v1/withdrawal/{}", id), None as Option<String>)
}

/// Retrieve withdrawals.
pub fn list_withdrawals(client: &Client) -> impl Future<Item = Vec<Withdrawal>, Error = Error> {
    client.get("/v1/withdrawals", None as Option<String>)
}

/// Create refund
pub fn create_refund(client: &Client, payload: refund::Payload) -> impl Future<Item = Refund, Error = Error> {
    client.post("/v1/refunds", Some(payload))
}

/// Retrieve refund with the given id
pub fn get_refund(client: &Client, id: &str) -> impl Future<Item = Refund, Error = Error> {
    client.get(format!("/v1/refund/{}", id), None as Option<String>)
}

/// Retrieve refunds.
pub fn list_refunds(client: &Client) -> impl Future<Item = Vec<Refund>, Error = Error> {
    client.get("/v1/refunds", None as Option<String>)
}

/// Retrieve available currencies.
pub fn list_currencies(client: &Client) -> impl Future<Item = Vec<Currency>, Error = Error> {
    client.get("/v1/currencies", None as Option<String>)
}

/// Retrieve account balance.
pub fn get_account_balance(client: &Client) -> impl Future<Item = Balance, Error = Error> {
    client.get("/v1/account/balance", None as Option<String>)
}

/// Retrieve rate list.
pub fn list_rates(client: &Client) -> impl Future<Item = Rates, Error = Error> {
    client.get("/v1/rates", None as Option<String>)
}
