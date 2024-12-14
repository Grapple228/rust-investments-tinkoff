use crate::{config, Result};
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};

/// Channel builder
#[derive(Debug, Clone)]
pub struct ChannelBuilder {
    endpoint: Endpoint,
}

impl ChannelBuilder {
    /// Creates channel builder with default tls config and default endpoint
    pub fn default() -> Result<Self> {
        let tls = ClientTlsConfig::new().with_native_roots();

        Self::new(&config().TINKOFF_API).tls(tls)
    }

    pub fn new(url: &'static str) -> Self {
        Self {
            endpoint: Channel::from_static(url),
        }
    }

    /// Sets channel link
    pub fn url(mut self, url: &'static str) -> Self {
        self.endpoint = Channel::from_static(url);
        self
    }

    /// Sets tls config
    pub fn tls(mut self, tls: ClientTlsConfig) -> Result<Self> {
        self.endpoint = self.endpoint.tls_config(tls)?;
        Ok(self)
    }

    /// Builds channel
    pub async fn build(self) -> Result<Channel> {
        Ok(self.endpoint.connect().await?)
    }
}
