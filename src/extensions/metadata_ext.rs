/// Extension trait for 'MetadataMap'
pub trait MetadataExt {
    /// Safely inserts data into metadata.
    /// # Warnings
    /// Sends a warning if the value cannot be parsed into a 'MetadataValue<Ascii>'
    /// # Returns
    /// `true` if the value was inserted successfully
    fn safe_append(
        &mut self,
        key: &'static str,
        value: impl Into<String>,
        err_message: impl Into<String>,
    ) -> bool;
}

impl MetadataExt for tonic::metadata::MetadataMap {
    fn safe_append(
        &mut self,
        key: &'static str,
        value: impl Into<String>,
        err_message: impl Into<String>,
    ) -> bool {
        let value: String = value.into();

        if let Ok(value) = value.parse::<tonic::metadata::MetadataValue<_>>() {
            self.append(key, value);

            true
        } else {
            tracing::warn!("{} '{}'", err_message.into(), value);

            false
        }
    }
}
