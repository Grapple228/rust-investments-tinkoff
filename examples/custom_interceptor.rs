use investments_tinkoff::api::v1::{GetAccountsRequest, InvestApiTrait};
use investments_tinkoff::channel::ChannelBuilder;
use investments_tinkoff::{api::v1::IntercemptorWithNew, extensions::MetadataExt};
use investments_tinkoff::{config, Result};
use tonic::service::Interceptor;

// region:    --- Custom Interceptor

pub struct CustomInterceptorData {
    token: String,
}

pub struct CustomInterceptor {
    data: CustomInterceptorData,
}

impl Interceptor for CustomInterceptor {
    fn call(
        &mut self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        let mut request = request;
        let metadata = request.metadata_mut();

        // Do something with request

        metadata.safe_append(
            "authorization",
            format!("Bearer {}", self.data.token),
            "Failed to insert token",
        );

        Ok(request)
    }
}

impl IntercemptorWithNew<CustomInterceptorData> for CustomInterceptor {
    fn new(data: CustomInterceptorData) -> Self {
        Self { data }
    }
}

// endregion: --- Custom Interceptor

// region:    --- Custom Api

pub struct CustomApi {
    token: String,
}

impl Default for CustomApi {
    /// Creates new CustomApi with default token from config
    fn default() -> Self {
        Self::with_token(&config().TINKOFF_TOKEN)
    }
}

impl InvestApiTrait<CustomInterceptorData, CustomInterceptor> for CustomApi {
    // region:    --- Constructors

    /// Creates new CustomApi with token
    fn with_token(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }

    // endregion: --- Constructors

    // region:    --- Getters

    fn interceptor_data(&self) -> CustomInterceptorData {
        CustomInterceptorData {
            token: self.token.clone(),
        }
    }

    // endregion: --- Getters
}

// endregion: --- Custom Api

#[tokio::main]
async fn main() -> Result<()> {
    // -- Create api
    let api = CustomApi::default();

    // -- Create channel
    let channel = ChannelBuilder::default()?.connect().await?;

    // -- Create users client
    let mut users_client = api.users(&channel)?;

    // -- Create request
    let request = tonic::Request::new(GetAccountsRequest { status: None });

    // -- Send request
    let response = users_client.get_accounts(request).await?;

    println!("RESPONSE={:#?}", response);

    Ok(())
}
