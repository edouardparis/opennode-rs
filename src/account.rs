use std::collections::HashMap;

use futures::future::Future;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::currency::Currency;
use crate::error::Error;

/// Balance is the account balance resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub balance: HashMap<Currency, f64>,
}

/// Retrieve account balance.
pub fn balance(client: &Client) -> impl Future<Item = Balance, Error = Error> {
    client.get("/v1/account/balance", None as Option<String>)
}
