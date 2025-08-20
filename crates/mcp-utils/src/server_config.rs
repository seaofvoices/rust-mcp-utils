use std::time::Duration;

#[derive(Debug, Clone)]
pub(crate) struct ServerConfig {
    pub(crate) name: String,
    pub(crate) title: String,
    pub(crate) version: String,
    pub(crate) instructions: String,
    pub(crate) timeout: Duration,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            title: "".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            instructions: "".to_string(),
            timeout: Duration::from_secs(60),
        }
    }
}
