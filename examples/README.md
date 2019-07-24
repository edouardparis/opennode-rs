# examples

## Charge

Create a new charge:
`cargo run --example charge -- --key=<KEY> create --amount=2000`

Get a charge with the given id:
`cargo run --example charge -- --key=<KEY> get <ID>`

List paid charges:
`cargo run --example charge -- --key=<KEY> list`

## Refund

Create a new refund:
`cargo run --example refund -- --key=<KEY> create --id=<ID> --address=<ADDRESS>`

Get a refund with the given id:
`cargo run --example refund -- --key=<KEY> get <ID>`

List paid refunds:
`cargo run --example refund -- --key=<KEY> list`

## Currencies

List available currencies:
`cargo run --example currencies -- --key=<KEY>`

