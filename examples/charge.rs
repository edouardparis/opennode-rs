use futures::future::{lazy};
use actix_rt::System;
use clap::{Arg, ArgMatches, App, SubCommand};

use opennode::client::Client;
use opennode::charge;

/// Create a new charge:
/// `cargo run --example charge -- --key=<KEY> create --amount=2000`
///
/// Get a charge with the given id:
/// `cargo run --example charge -- --key=<KEY> get <ID>`
///
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
                         .takes_value(true)))
       .subcommand(SubCommand::with_name("get")
                    .about("retrieve a charge with the given id")
                    .arg(Arg::with_name("id")
                         .value_name("ID")
                         .help("id of the charge")
                         .required(true)
                         .takes_value(true)));

    let matches = app.get_matches();
    let api_key = matches.value_of("key").unwrap();
    let client = Client::from_url("https://dev-api.opennode.co", api_key);

    match matches.subcommand() {
        ("create", Some(m)) => create(m, &client),
        ("get", Some(m)) => get(m, &client),
        _ => (),
    }
}

fn create(matches: &ArgMatches, client: &Client) {
    let a = matches.value_of("amount").unwrap();
    let amount = a.parse::<u64>().unwrap();
    let charge: charge::Charge = System::new("test").block_on(lazy(|| {
        charge::create(&client, charge::Payload::new(amount))
    })).unwrap();

    println!("{:?}", charge)
}

fn get(matches: &ArgMatches, client: &Client) {
    let id = matches.value_of("id").unwrap();
    let charge: charge::Charge = System::new("test").block_on(lazy(|| {
        charge::get(&client, id)
    })).unwrap();

    println!("{:?}", charge)
}
