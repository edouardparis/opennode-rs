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
opennode-client = "0.1.0"
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

To get started, create a client:

```rust
let client = opennode_client::client::Client::new("OPENNODE-TOKEN");
```

Let's create a new charge using an actor system like [actix_rt](https://crates.io/crates/actix-rt):

```rust
use opennode::charge;
use opennode_client::create_charge;

// opennode_client::create_charge signature:
// (client: &Client, payload: Payload) -> impl Future<Item=Charge, Error=Error>

let charge: charge::Charge = System::new("test").block_on(lazy(|| {
    create_charge(&client, charge::Payload::new(1000))
})).unwrap();
```
