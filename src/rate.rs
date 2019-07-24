use std::collections::HashMap;

use futures::future::Future;

use crate::client::Client;
use crate::error::Error;
use crate::currency::Currency;

/// Rates is the rates resource.
pub type Rates = HashMap<String, HashMap<Currency, f64>>;

/// Retrieve rate list.
pub fn list(client: &Client) -> impl Future<Item=Rates, Error=Error> {
    client.get("/rates", None as Option<String>)
}
