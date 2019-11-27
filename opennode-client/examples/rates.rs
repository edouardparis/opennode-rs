use opennode::rate;
use opennode_client::client::Client;
use opennode_client::list_rates;

/// List currency:
/// `cargo run --example rates`
///
#[tokio::main]
async fn main() {
    let client = Client::from_url("https://dev-api.opennode.co", "");
    let list: rate::Rates = list_rates(&client).await.unwrap();
    println!("{:?}", list)
}
