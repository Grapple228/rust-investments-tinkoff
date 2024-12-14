# About

Client for Tinkoff Invesments API over gRPC protocol

## Setup

For compiling protos, run

```sh
cargo run --example build_contracts
```

## Example

Add to your Cargo.toml

```toml
[dependencies]
investments_tinkoff = "0.2"
```

Then use it like this:

```rust
    // /examples/simple.rs

    // -- Create api
    let api = InvestApi::default().with_app_name("Grapple228.rust-investments-tinkoff");

    // -- Create channel
    let channel = ChannelBuilder::default()?.connect().await?;

    // -- Create users client
    let mut users_client = api.users(&channel).await?;

    // -- Create request
    let request = tonic::Request::new(GetAccountsRequest { status: None });

    // -- Send request
    let response = users_client.get_accounts(request).await?;

    println!("RESPONSE={:#?}", response);

    Ok(())

```

## License

Licensed under MIT license ([LICENSE](LICENSE) or [LINK](http://opensource.org/licenses/MIT))
