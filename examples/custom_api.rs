use investments_tinkoff::api::v1::{
    GetAccountsRequest, InterceptorData, InvestApiTrait, TinkoffInterceptor,
};
use investments_tinkoff::channel::ChannelBuilder;
use investments_tinkoff::{config, Result};

// region:    --- Custom Api

pub struct CustomApi {
    token: String,
    app_name: Option<String>,
}

impl CustomApi {
    /// Sets application name
    pub fn with_app_name(mut self, app_name: impl Into<String>) -> Self {
        self.app_name = Some(app_name.into());
        self
    }
}

impl Default for CustomApi {
    /// Creates new CustomApi with default token from config
    fn default() -> Self {
        Self::with_token(&config().TINKOFF_TOKEN)
    }
}

impl InvestApiTrait<InterceptorData, TinkoffInterceptor> for CustomApi {
    // region:    --- Constructors

    /// Creates new CustomApi with token
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

// endregion: --- Custom Api

#[tokio::main]
async fn main() -> Result<()> {
    // -- Create api
    let api = CustomApi::default().with_app_name("Grapple228.rust-investments-tinkoff");

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
