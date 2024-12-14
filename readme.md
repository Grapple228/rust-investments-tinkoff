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
investments_tinkoff = "0.1"
```

Then use it like this:

```rust
    // \examples\simple.rs

    // -- Init logging and config
    _ = investments_tinkoff::init();

    // -- Create api
    let api = TinkoffApi::default().with_app_name("Grapple228.rust-investments-tinkoff");

    // -- Create channel
    let channel = ChannelBuilder::default()?.build().await?;

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

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
