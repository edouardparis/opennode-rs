use actix_rt::System;
use futures::future::lazy;

use opennode::rate;
use opennode_client::client::Client;
use opennode_client::list_rates;

/// List currency:
/// `cargo run --example rates`
///
fn main() {
    let client = Client::from_url("https://dev-api.opennode.co", "");

    let list: rate::Rates = System::new("test")
        .block_on(lazy(|| list_rates(&client)))
        .unwrap();

    println!("{:?}", list)
}
