use futures::future::{lazy};
use actix_rt::System;
use clap::{Arg, App};

use opennode::client::Client;
use opennode::currency;

/// List currency:
/// `cargo run --example currencies -- --key=<KEY>`
///
fn main() {
    let app = App::new("currencies")
        .arg(Arg::with_name("key")
             .short("k")
             .long("key")
             .help("opennode api_key")
             .value_name("KEY")
             .required(true)
             .takes_value(true));

    let matches = app.get_matches();
    let api_key = matches.value_of("key").unwrap();
    let client = Client::from_url("https://dev-api.opennode.co", api_key);

    let currencies: Vec<currency::Currency> = System::new("test").block_on(lazy(|| {
        currency::list(&client)
    })).unwrap();

    println!("{:?}", currencies)
}
