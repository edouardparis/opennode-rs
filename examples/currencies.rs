use actix_rt::System;
use futures::future::lazy;

use opennode::client::Client;
use opennode::currency;

/// List currency:
/// `cargo run --example currencies`
///
fn main() {
    let client = Client::from_url("https://dev-api.opennode.co", "");

    let currencies: Vec<currency::Currency> = System::new("test")
        .block_on(lazy(|| currency::list(&client)))
        .unwrap();

    println!("{:?}", currencies)
}
