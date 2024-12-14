use investments_tinkoff::{
    api::v1::{GetAccountsRequest, InvestApi},
    channel::ChannelBuilder,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
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
}
