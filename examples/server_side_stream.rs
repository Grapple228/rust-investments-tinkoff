use investments_tinkoff::{
    api::v1::{
        CandleInstrument, InvestApi, InvestApiTrait, MarketDataServerSideStreamRequest,
        PingDelaySettings, SubscribeCandlesRequest, SubscriptionAction, SubscriptionInterval,
    },
    channel::ChannelBuilder,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // -- Create api
    let api = InvestApi::default().with_app_name("Grapple228.rust-investments-tinkoff");

    // -- Create channel
    let channel = ChannelBuilder::default()?.connect().await?;

    // -- Create market data stream
    let mut market_data_stream = api.market_data_stream(&channel)?;

    // -- Create request

    let request = SubscribeCandlesRequest {
        subscription_action: SubscriptionAction::Subscribe as i32,

        #[allow(deprecated)]
        instruments: vec![CandleInstrument {
            // figi is deprecated, so just pass empty string
            figi: String::new(),
            interval: SubscriptionInterval::OneMinute as i32,
            instrument_id: "BBG000B9XRY4".to_string(),
        }],

        waiting_close: true,
        candle_source_type: None,
    };

    let request = MarketDataServerSideStreamRequest {
        subscribe_candles_request: Some(request),
        ping_settings: Some(PingDelaySettings {
            ping_delay_ms: Some(1000),
        }),
        ..Default::default()
    };

    // -- Send request
    let mut stream = market_data_stream
        .market_data_server_side_stream(request)
        .await?
        .into_inner();

    while let Some(response) = stream.message().await? {
        println!("{:#?}", response);
    }

    Ok(())
}
