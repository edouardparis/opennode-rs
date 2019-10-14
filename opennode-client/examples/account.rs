use actix_rt::System;
use clap::{App, Arg};
use futures::future::lazy;

use opennode::account;
use opennode_client::client::Client;

/// List currency:
/// `cargo run --example account -- --key=<KEY>`
///
fn main() {
    let app = App::new("account").arg(
        Arg::with_name("key")
            .short("k")
            .long("key")
            .help("opennode api_key")
            .value_name("KEY")
            .required(true)
            .takes_value(true),
    );

    let matches = app.get_matches();
    let api_key = matches.value_of("key").unwrap();
    let client = Client::from_url("https://dev-api.opennode.co", api_key);

    let balance: account::Balance = System::new("test")
        .block_on(lazy(|| opennode_client::get_account_balance(&client)))
        .unwrap();

    println!("{:?}", balance)
}
