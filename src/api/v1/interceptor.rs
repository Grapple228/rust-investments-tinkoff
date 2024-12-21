// region:    --- Modules

use tonic::service::Interceptor;
use uuid::Uuid;

use crate::extensions::MetadataExt;

// endregion: --- Modules

// region:    --- Interceptor Data

/// Interceptor data for modifying request
pub struct InterceptorData {
    pub token: String,
    pub app_name: Option<String>,
}

// endregion: --- Interceptor Data

/// Interceptor that requires new method
pub trait IntercemptorWithNew<D>: Interceptor {
    /// Creates new interceptor
    fn new(data: D) -> Self;
}

/// Custom implementation for Tinkoff Interceptor
pub struct TinkoffInterceptor {
    pub data: InterceptorData,
}

impl IntercemptorWithNew<InterceptorData> for TinkoffInterceptor {
    fn new(data: InterceptorData) -> Self {
        Self { data }
    }
}

impl Interceptor for TinkoffInterceptor {
    fn call(
        &mut self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        let mut request = request;
        let metadata = request.metadata_mut();

        metadata.safe_append(
            "authorization",
            format!("Bearer {}", self.data.token),
            "Failed to insert token",
        );

        metadata.safe_append(
            "x-request-id",
            Uuid::new_v4().to_string(),
            "Failed to insert request id",
        );

        if let Some(app_name) = &self.data.app_name {
            metadata.safe_append("x-app-name", app_name, "Failed to insert app name");
        }

        Ok(request)
    }
}
