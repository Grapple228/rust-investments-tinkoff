// region:    --- Modules

// Modules
#[path = "google.api.rs"]
pub mod google_api;

mod interceptor;
mod protos;

// -- Flatten
pub use interceptor::TinkoffInterceptor;
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
        pub async fn $func_name(&self, channel: &Channel) -> Result<$client_type> {
            let interceptor = <$interceptor_type>::new(self.token.clone(), self.app_name.clone());
            let intercepted_channel = InterceptedService::new(channel.clone(), interceptor);

            Ok(<$client_type>::new(intercepted_channel))
        }
    };
    ($func_name:ident, $client_type:ty) => {
        pub async fn $func_name(&self, channel: &Channel) -> Result<$client_type> {
            let interceptor = TinkoffInterceptor::new(self.token.clone(), self.app_name.clone());
            let intercepted_channel = InterceptedService::new(channel.clone(), interceptor);

            Ok(<$client_type>::new(intercepted_channel))
        }
    };
}

// endregion: --- Macroses

/// Tinkoff API client to communicate with Tinkoff Invest API
/// # Fields
/// * `token` - API token
/// * `app_name` - application name
#[derive(Debug, Clone)]
pub struct InvestApi {
    token: String,
    app_name: std::option::Option<String>,
}

impl Default for InvestApi {
    fn default() -> Self {
        Self::with_token(&config().TINKOFF_TOKEN)
    }
}

impl InvestApi {
    // region:    --- Constructors

    /// Creates new TinkoffApi with token
    pub fn with_token(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            app_name: None,
        }
    }

    /// Sets application name
    pub fn with_app_name(mut self, app_name: impl Into<String>) -> Self {
        self.app_name = Some(app_name.into());
        self
    }

    // endregion: --- Constructors

    // region:    --- Clients

    generate_client!(
        instruments,
        InstrumentsServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    generate_client!(
        market_data,
        MarketDataServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    generate_client!(
        market_data_stream,
        MarketDataStreamServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    generate_client!(
        operations,
        OperationsServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    generate_client!(
        orders,
        OrdersServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    generate_client!(
        orders_stream,
        OrdersStreamServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    generate_client!(
        sandbox,
        SandboxServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    generate_client!(
        signal,
        SignalServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    generate_client!(
        stop_orders,
        StopOrdersServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    generate_client!(
        users,
        UsersServiceClient<InterceptedService<Channel, TinkoffInterceptor>>
    );

    // endregion: --- Clients
}
