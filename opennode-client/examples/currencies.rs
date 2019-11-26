use opennode::currency;
use opennode_client::client::Client;
use opennode_client::list_currencies;

/// List currency:
/// `cargo run --example currencies`
#[tokio::main]
fn main() {
    let client = Client::from_url("https://dev-api.opennode.co", "");

    let currencies: Vec<currency::Currency> = list_currencies(&client).await.unwrap();

    println!("{:?}", currencies)
}
