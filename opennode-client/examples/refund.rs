use clap::{App, Arg, ArgMatches, SubCommand};
use opennode::refund;
use opennode_client::*;
use opennode_client::client::Client;

/// Create a new refund:
/// `cargo run --example refund -- --key=<KEY> create --id=<ID> --address=<ADDRESS>`
///
/// Get a refund with the given id:
/// `cargo run --example refund -- --key=<KEY> get <ID>`
///
/// List paid refunds:
/// `cargo run --example refund -- --key=<KEY> list`
#[tokio::main]
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
        ("create", Some(m)) => create(m, &client).await,
        ("get", Some(m)) => get(m, &client).await,
        ("list", _) => list(&client).await,
        _ => (),
    }
}

async fn create(matches: &ArgMatches, client: &Client) {
    let id = matches.value_of("charge_id").unwrap();
    let address = matches.value_of("address").unwrap();
    let refund: refund::Refund = create_refund(&client, refund::Payload::new(id, address))
        .await
        .unwrap();

    println!("{:?}", refund)
}

async fn get(matches: &ArgMatches, client: &Client) {
    let id = matches.value_of("id").unwrap();
    let refund: refund::Refund = get_refund(&client, id)
        .await
        .unwrap();

    println!("{:?}", refund)
}

async fn list(client: &Client) {
    let refunds: Vec<refund::Refund> = list_refunds(&client)
        .await
        .unwrap();

    println!("{:?}", refunds)
}
