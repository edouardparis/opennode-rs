use actix_rt::System;
use clap::{App, Arg, ArgMatches, SubCommand};
use futures::future::lazy;

use opennode_client::client::Client;
use opennode_client::*;
use opennode::withdrawal;

/// Create a new withdrawal with a lightning invoice:
/// `cargo run --example withdrawal -- --key=<KEY> create --invoice=<INVOICE>`
///
/// Create a new withdrawal with a bitcoin address:
/// `cargo run --example withdrawal -- --key=<KEY> create --address=<ADDRESS> --amount=<AMOUNT>`
///
/// Get a withdrawal with the given id:
/// `cargo run --example withdrawal -- --key=<KEY> get <ID>`
///
/// List paid withdrawals:
/// `cargo run --example withdrawal -- --key=<KEY> list`
///
fn main() {
    let app = App::new("withdrawal")
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
                .about("creates a new withdrawal")
                .arg(
                    Arg::with_name("amount")
                        .short("amt")
                        .long("amount")
                        .value_name("AMOUNT")
                        .help("withdrawal amount in satoshis")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("address")
                        .long("address")
                        .value_name("ADDRESS")
                        .help("withdrawal address on chain")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("invoice")
                        .long("invoice")
                        .value_name("INVOICE")
                        .help("withdrawal invoice")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("retrieve a withdrawal with the given id")
                .arg(
                    Arg::with_name("id")
                        .value_name("ID")
                        .help("id of the withdrawal")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("retrieve withdrawals"));

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
    let amount: Option<u64> = match matches.value_of("amount") {
        Some(v) => Some(v.parse::<u64>().unwrap()),
        None => None,
    };

    let kind = match matches.value_of("invoice") {
        Some(_) => withdrawal::Type::Ln,
        None => withdrawal::Type::Chain,
    };

    let address: &str = match kind {
        withdrawal::Type::Ln => matches.value_of("invoice").unwrap(),
        withdrawal::Type::Chain => matches.value_of("address").unwrap(),
    };

    let withdrawal: withdrawal::Withdrawal = System::new("test")
        .block_on(lazy(|| {
            create_withdrawal(&client, withdrawal::Payload::new(kind, address, amount))
        }))
        .unwrap();

    println!("{:?}", withdrawal)
}

fn get(matches: &ArgMatches, client: &Client) {
    let id = matches.value_of("id").unwrap();
    let withdrawal: withdrawal::Withdrawal = System::new("test")
        .block_on(lazy(|| get_withdrawal(&client, id)))
        .unwrap();

    println!("{:?}", withdrawal)
}

fn list(client: &Client) {
    let withdrawals: Vec<withdrawal::Withdrawal> = System::new("test")
        .block_on(lazy(|| list_withdrawals(&client)))
        .unwrap();

    println!("{:?}", withdrawals)
}
