use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    pub ak: String,
    pub sk: String,
}