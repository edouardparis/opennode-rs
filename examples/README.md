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

## Withdrawal

Create a new withdrawal with a lightning invoice:
`cargo run --example withdrawal -- --key=<KEY> create --invoice=<INVOICE>`

Create a new withdrawal with a bitcoin address:
`cargo run --example withdrawal -- --key=<KEY> create --address=<ADDRESS> --amount=<AMOUNT>`

Get a withdrawal with the given id:
`cargo run --example withdrawal -- --key=<KEY> get <ID>`

List paid withdrawals:
`cargo run --example withdrawal -- --key=<KEY> list`

## Account

Retrieve account balance:
`cargo run --example account -- --key=<KEY>`

## Currencies

List supported currencies:
`cargo run --example currencies`

## Rates

List current rates:
`cargo run --example rates`
