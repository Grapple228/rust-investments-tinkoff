use tonic::{
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
};
use tracing::warn;
use uuid::Uuid;

use crate::extensions::MetadataExt;

pub struct TinkoffInterceptor {
    token: String,
    app_name: Option<String>,
}

impl TinkoffInterceptor {
    pub fn new(token: String, app_name: Option<String>) -> Self {
        Self { token, app_name }
    }
}

impl Interceptor for TinkoffInterceptor {
    fn call(
        &mut self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        let mut request = request;
        let mut metadata = request.metadata_mut();

        metadata.safe_append("authorization", &self.token, "Failed to insert token");

        metadata.safe_append(
            "x-request-id",
            &Uuid::new_v4().to_string(),
            "Failed to insert request id",
        );

        if let Some(app_name) = &self.app_name {
            metadata.safe_append("x-app-name", app_name, "Failed to insert app name");
        }

        Ok(request)
    }
}
