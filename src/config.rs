use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ClientConfig {
    pub ak: String,
    pub sk: String,
}
