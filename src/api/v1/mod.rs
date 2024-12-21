// region:    --- Modules

// Modules
#[path = "google.api.rs"]
pub mod google_api;

mod interceptor;
mod protos;

// -- Flatten
pub use interceptor::{IntercemptorWithNew, InterceptorData, TinkoffInterceptor};
pub use protos::*;

// -- Use
use crate::{config, Result};
use instruments_service_client::InstrumentsServiceClient;
use market_data_service_client::MarketDataServiceClient;
use market_data_stream_service_client::MarketDataStreamServiceClient;
use operations_service_client::OperationsServiceClient;
use orders_service_client::OrdersServiceClient;
use orders_stream_service_client::OrdersStreamServiceClient;
use sandbox_service_client::SandboxServiceClient;
use signal_service_client::SignalServiceClient;
use stop_orders_service_client::StopOrdersServiceClient;
use tonic::{service::interceptor::InterceptedService, transport::Channel};
use users_service_client::UsersServiceClient;

// endregion: --- Modules

// region:    --- Macroses

/// Macro for generating clients.
/// # Arguments
/// * `func_name` - name of the function
/// * `client_type` - type of the client
/// * `interceptor_type` - type of the interceptor. Is optional, default is `TinkoffInterceptor`.
#[macro_export]
macro_rules! generate_client {
    ($func_name:ident, $client_type:ty, $interceptor_type:ty) => {
        fn $func_name(&self, channel: &Channel) -> Result<$client_type> {
            let interceptor = <$interceptor_type>::new(self.interceptor_data());
            let intercepted_channel = InterceptedService::new(channel.clone(), interceptor);

            Ok(<$client_type>::new(intercepted_channel))
        }
    };
    ($func_name:ident, $client_type:ty) => {
        fn $func_name(&self, channel: &Channel) -> Result<$client_type> {
            let interceptor = TinkoffInterceptor::new(self.interceptor_data());
            let intercepted_channel = InterceptedService::new(channel.clone(), interceptor);

            Ok(<$client_type>::new(intercepted_channel))
        }
    };
}

// endregion: --- Macroses

// region:    --- Trait

pub trait InvestApiTrait<D, I: IntercemptorWithNew<D>> {
    // region:    --- Constructors

    /// Creates new Api with token
    fn with_token(token: impl Into<String>) -> Self;

    // endregion: --- Constructors

    // region:    --- Getters

    /// Creates interceptor data for modifying request
    fn interceptor_data(&self) -> D;

    // endregion: --- Getters

    // region:    --- Clients
    generate_client!(
        instruments,
        InstrumentsServiceClient<InterceptedService<Channel, I>>,
        I
    );

    generate_client!(
        market_data,
        MarketDataServiceClient<InterceptedService<Channel, I>>,
        I
    );

    generate_client!(
        market_data_stream,
        MarketDataStreamServiceClient<InterceptedService<Channel, I>>,
        I
    );

    generate_client!(
        operations,
        OperationsServiceClient<InterceptedService<Channel, I>>,
        I
    );

    generate_client!(
        orders,
        OrdersServiceClient<InterceptedService<Channel, I>>,
        I
    );

    generate_client!(
        orders_stream,
        OrdersStreamServiceClient<InterceptedService<Channel, I>>,
        I
    );

    generate_client!(
        sandbox,
        SandboxServiceClient<InterceptedService<Channel, I>>,
        I
    );

    generate_client!(
        signal,
        SignalServiceClient<InterceptedService<Channel, I>>,
        I
    );

    generate_client!(
        stop_orders,
        StopOrdersServiceClient<InterceptedService<Channel, I>>,
        I
    );

    generate_client!(users, UsersServiceClient<InterceptedService<Channel, I>>, I);

    // endregion: --- Clients
}

// endregion: --- Trait

// region:    --- Invest Api

/// Tinkoff API client to communicate with Tinkoff Invest API
/// # Fields
/// * `token` - API token
/// * `app_name` - application name
#[derive(Debug, Clone)]
pub struct InvestApi {
    pub token: String,
    pub app_name: std::option::Option<String>,
}

impl Default for InvestApi {
    /// Creates new TinkoffApi with default token from config
    fn default() -> Self {
        Self::with_token(&config().TINKOFF_TOKEN)
    }
}

impl InvestApiTrait<InterceptorData, TinkoffInterceptor> for InvestApi {
    // region:    --- Constructors

    /// Creates new TinkoffApi with token
    fn with_token(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            app_name: None,
        }
    }

    // endregion: --- Constructors

    // region:    --- Getters

    fn interceptor_data(&self) -> InterceptorData {
        InterceptorData {
            token: self.token.clone(),
            app_name: self.app_name.clone(),
        }
    }

    // endregion: --- Getters
}

impl InvestApi {
    /// Sets application name
    pub fn with_app_name(mut self, app_name: impl Into<String>) -> Self {
        self.app_name = Some(app_name.into());
        self
    }
}

// endregion: --- Invest Api
