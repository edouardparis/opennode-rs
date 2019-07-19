use futures::future::{lazy};
use actix_rt::System;
use clap::{Arg, App, SubCommand};

use opennode::client::Client;
use opennode::charge;


/// cargo run --example charge -- --key=<KEY> create --amount=2000
fn main() {
    let app = App::new("charge")
        .arg(Arg::with_name("key")
             .short("k")
             .long("key")
             .help("opennode api_key")
             .value_name("KEY")
             .required(true)
             .takes_value(true))
        .subcommand(SubCommand::with_name("create")
                    .about("creates a new charge")
                    .arg(Arg::with_name("amount")
                         .short("a")
                         .value_name("AMOUNT")
                         .help("charge amount in satoshis")
                         .required(true)
                         .takes_value(true)));

    let matches = app.get_matches();
    let api_key = matches.value_of("key").unwrap();


    let client = Client::from_url("https://dev-api.opennode.co", api_key);

    let mut test = System::new("test");

    let charge: charge::Charge = test.block_on(lazy(|| {
        charge::create(&client, charge::Payload::new(2000))
    })).unwrap();

    println!("{:?}", charge)
}
