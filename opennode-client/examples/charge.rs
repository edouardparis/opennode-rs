use clap::{App, Arg, ArgMatches, SubCommand};

use opennode::charge;
use opennode_client::*;
use opennode_client::client::Client;

/// Create a new charge:
/// `cargo run --example charge -- --key=<KEY> create --amount=2000`
///
/// Get a charge with the given id:
/// `cargo run --example charge -- --key=<KEY> get <ID>`
///
/// List paid charges:
/// `cargo run --example charge -- --key=<KEY> list`
#[tokio::main]
async fn main() {
    let app = App::new("charge")
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
                .about("creates a new charge")
                .arg(
                    Arg::with_name("amount")
                        .short("a")
                        .value_name("AMOUNT")
                        .help("charge amount in satoshis")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("retrieve a charge with the given id")
                .arg(
                    Arg::with_name("id")
                        .value_name("ID")
                        .help("id of the charge")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("retrieve paid charges"));

    let matches = app.get_matches();
    let api_key = matches.value_of("key").unwrap();
    let client = Client::from_url("https://dev-api.opennode.co", api_key);

    match matches.subcommand() {
        ("create", Some(m)) => create(m, &client).await,
        ("get", Some(m)) => get(m, &client).await,
        ("list", _) => list(&client).await,
        _ => (),
    }
}

async fn create(matches: &ArgMatches<'_>, client: &Client) {
    let a = matches.value_of("amount").unwrap();
    let amount = a.parse::<u64>().unwrap();
    let charge: charge::Charge = create_charge(&client, charge::Payload::new(amount)).await.unwrap();

    println!("{:?}", charge)
}

async fn get(matches: &ArgMatches<'_>, client: &Client) {
    let id = matches.value_of("id").unwrap();
    let charge: charge::Charge = get_charge(&client, id).await.unwrap();

    println!("{:?}", charge)
}

async fn list(client: &Client) {
    let charges: Vec<charge::Charge> = list_charges(&client).await.unwrap();

    println!("{:?}", charges)
}
