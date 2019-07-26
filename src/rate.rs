use std::collections::HashMap;

use futures::future::Future;

use crate::client::Client;
use crate::currency::Currency;
use crate::error::Error;

/// Rates is the rates resource.
pub type Rates = HashMap<String, HashMap<Currency, f64>>;

/// Retrieve rate list.
pub fn list(client: &Client) -> impl Future<Item = Rates, Error = Error> {
    client.get("/v1/rates", None as Option<String>)
}
