# opennode

Rust API bindings for the Opennode v1 HTTP API.
This library rely on rust Futures to allow asynchronous usage.

[Opennode API documentation](https://developers.opennode.co)

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
opennode = "0.1.0"
```

And this in your crate root:

```rust
extern crate opennode;
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
let client = opennode::client::Client::new("OPENNODE-TOKEN");
```

Let's create a new charge using an actor system like [actix_rt](https://crates.io/crates/actix-rt):

```rust
use opennode::charge;

// opennode::charge::create signature:
// (client: &Client, payload: Payload) -> impl Future<Item=Charge, Error=Error>

let charge: charge::Charge = System::new("test").block_on(lazy(|| {
    charge::create(&client, charge::Payload::new(1000))
})).unwrap();
```

