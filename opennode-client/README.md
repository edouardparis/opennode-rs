# opennode-client

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/edouardparis/opennode-rs/blob/master/LICENSE)
[![opennode on crates.io](https://img.shields.io/crates/v/opennode-client.svg)](https://crates.io/crates/opennode-client)
[![opennode on docs.rs](https://docs.rs/opennode-client/badge.svg)](https://docs.rs/opennode-client)
[![tippin.me](https://badgen.net/badge/%E2%9A%A1%EF%B8%8Ftippin.me/@edouardparis/F0918E)](https://tippin.me/@edouardparis)

Rust Client for the Opennode v1 HTTP API.
This library rely on rust Futures to allow asynchronous usage.

[Opennode API documentation](https://developers.opennode.co)

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
opennode = "1.0.0"
opennode-client = "1.0.1"
```

And this in your crate root:

```rust
extern crate opennode;
extern crate opennode_client;
```

## Test

```
cargo test
```

## Examples

Run file from [examples](./examples) with:

```
cargo run --example <example> -- <example flags> <example args>
```

## Getting Started

```rust
use clap::{App, Arg};

use opennode::account;
use opennode_client::{client::Client, get_account_balance};

/// Get account balance:
/// `cargo run --example account -- --key=<KEY>`
#[tokio::main]
async fn main() {
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

    let balance: account::Balance = get_account_balance(&client).await.unwrap();

    println!("{:?}", balance)
}
```
