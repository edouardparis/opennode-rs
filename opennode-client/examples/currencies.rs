use actix_rt::System;
use futures::future::lazy;

use opennode_client::client::Client;
use opennode_client::list_currencies;
use opennode::currency;

/// List currency:
/// `cargo run --example currencies`
///
fn main() {
    let client = Client::from_url("https://dev-api.opennode.co", "");

    let currencies: Vec<currency::Currency> = System::new("test")
        .block_on(lazy(|| list_currencies(&client)))
        .unwrap();

    println!("{:?}", currencies)
}
