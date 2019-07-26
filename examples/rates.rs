use actix_rt::System;
use futures::future::lazy;

use opennode::client::Client;
use opennode::rate;

/// List currency:
/// `cargo run --example rates`
///
fn main() {
    let client = Client::from_url("https://dev-api.opennode.co", "");

    let list: rate::Rates = System::new("test")
        .block_on(lazy(|| rate::list(&client)))
        .unwrap();

    println!("{:?}", list)
}
