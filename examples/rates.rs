use futures::future::{lazy};
use actix_rt::System;

use opennode::client::Client;
use opennode::rate;

/// List currency:
/// `cargo run --example rates`
///
fn main() {
    let client = Client::from_url("https://dev-api.opennode.co", "");

    let list: rate::Rates = System::new("test").block_on(lazy(|| {
        rate::list(&client)
    })).unwrap();

    println!("{:?}", list)
}
