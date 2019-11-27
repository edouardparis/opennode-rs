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

use crate::client::Client;
use crate::error::Error;

/// Create charge
pub async fn create_charge(client: &Client, payload: charge::Payload) -> Result<Charge, Error> {
    client.post("/v1/charges", Some(payload)).await
}

/// Retrieve charge with the given id
pub async fn get_charge(client: &Client, id: &str) -> Result< Charge, Error> {
    let path = format!("/v1/charge/{}", id);
    client.get(&path, None as Option<String>).await
}

/// Retrieve paid charges.
pub async fn list_charges(client: &Client) -> Result<Vec<Charge>, Error> {
    client.get("/v1/charges", None as Option<String>).await
}

/// Create withdrawal
pub async fn create_withdrawal(client: &Client, payload: withdrawal::Payload) -> Result< Withdrawal,  Error> {
    client.post("/v2/withdrawals", Some(payload)).await
}

/// Retrieve withdrawal with the given id
pub async fn get_withdrawal(client: &Client, id: &str) -> Result<Withdrawal, Error> {
    let path = format!("/v1/withdrawal/{}", id);
    client.get(&path, None as Option<String>).await
}

/// Retrieve withdrawals.
pub async fn list_withdrawals(client: &Client) -> Result<Vec<Withdrawal>, Error> {
    client.get("/v1/withdrawals", None as Option<String>).await
}

/// Create refund
pub async fn create_refund(client: &Client, payload: refund::Payload) -> Result<Refund, Error> {
    client.post("/v1/refunds", Some(payload)).await
}

/// Retrieve refund with the given id
pub async fn get_refund(client: &Client, id: &str) -> Result<Refund, Error> {
    let path = format!("/v1/refund/{}", id);
    client.get(&path, None as Option<String>).await
}

/// Retrieve refunds.
pub async fn list_refunds(client: &Client) -> Result<Vec<Refund>, Error> {
    client.get("/v1/refunds", None as Option<String>).await
}

/// Retrieve available currencies.
pub async fn list_currencies(client: &Client) -> Result<Vec<Currency>, Error> {
    client.get("/v1/currencies", None as Option<String>).await
}

/// Retrieve account balance.
pub async fn get_account_balance(client: &Client) -> Result<Balance, Error> {
    client.get("/v1/account/balance", None as Option<String>).await
}

/// Retrieve rate list.
pub async fn list_rates(client: &Client) -> Result<Rates,  Error> {
    client.get("/v1/rates", None as Option<String>).await
}
