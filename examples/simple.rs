#![allow(unused)] // For beginning only.

use std::{borrow::BorrowMut, str::FromStr};

use investments_tinkoff::{
    api::v1::{users_service_client::UsersServiceClient, GetAccountsRequest, TinkoffApi},
    channel::ChannelBuilder,
    config, generate_client, Result,
};
use tonic::{
    metadata::MetadataValue,
    service::{interceptor::InterceptedService, Interceptor},
    transport::{channel, Channel, ClientTlsConfig, Identity},
    Request,
};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
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
}
