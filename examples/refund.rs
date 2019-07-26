use actix_rt::System;
use clap::{App, Arg, ArgMatches, SubCommand};
use futures::future::lazy;

use opennode::client::Client;
use opennode::refund;

/// Create a new refund:
/// `cargo run --example refund -- --key=<KEY> create --id=<ID> --address=<ADDRESS>`
///
/// Get a refund with the given id:
/// `cargo run --example refund -- --key=<KEY> get <ID>`
///
/// List paid refunds:
/// `cargo run --example refund -- --key=<KEY> list`
///
fn main() {
    let app = App::new("refund")
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .help("opennode api_key")
                .value_name("KEY")
                .required(true)
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("creates a new refund")
                .arg(
                    Arg::with_name("charge_id")
                        .value_name("ID")
                        .help("id of the underpaid charge")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("address")
                        .value_name("<BTC_ADDRESS>")
                        .help("address for the refund")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("retrieve a refund with the given id")
                .arg(
                    Arg::with_name("id")
                        .value_name("ID")
                        .help("id of the refund")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("retrieve refunds"));

    let matches = app.get_matches();
    let api_key = matches.value_of("key").unwrap();
    let client = Client::from_url("https://dev-api.opennode.co", api_key);

    match matches.subcommand() {
        ("create", Some(m)) => create(m, &client),
        ("get", Some(m)) => get(m, &client),
        ("list", _) => list(&client),
        _ => (),
    }
}

fn create(matches: &ArgMatches, client: &Client) {
    let id = matches.value_of("charge_id").unwrap();
    let address = matches.value_of("address").unwrap();
    let refund: refund::Refund = System::new("test")
        .block_on(lazy(|| {
            refund::create(&client, refund::Payload::new(id, address))
        }))
        .unwrap();

    println!("{:?}", refund)
}

fn get(matches: &ArgMatches, client: &Client) {
    let id = matches.value_of("id").unwrap();
    let refund: refund::Refund = System::new("test")
        .block_on(lazy(|| refund::get(&client, id)))
        .unwrap();

    println!("{:?}", refund)
}

fn list(client: &Client) {
    let refunds: Vec<refund::Refund> = System::new("test")
        .block_on(lazy(|| refund::list(&client)))
        .unwrap();

    println!("{:?}", refunds)
}
