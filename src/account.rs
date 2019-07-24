use std::collections::HashMap;

use futures::future::Future;
use serde::{Serialize, Deserialize};

use crate::client::Client;
use crate::error::Error;
use crate::currency::Currency;

/// Balance is the account balance resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub balance: HashMap<Currency, f64>
}

/// Retrieve account balance.
pub fn balance(client: &Client) -> impl Future<Item=Balance, Error=Error> {
    client.get("/account/balance", None as Option<String>)
}
