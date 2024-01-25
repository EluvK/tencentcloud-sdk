use std::sync::Arc;

use reqwest::header::{self};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};

mod signature;
use signature::{SignatureContext, SignatureMiddleware};

use crate::config::ClientConfig;

mod constant;
pub mod cvm;

pub use constant::*;

#[derive(Debug, Clone)]
pub struct TencentCloudClient {
    client: Arc<TencentCloudBaseClient>,
}

impl TencentCloudClient {
    pub fn new(config: &ClientConfig) -> Self {
        Self {
            client: Arc::new(TencentCloudBaseClient::new(
                config.ak.clone(),
                config.sk.clone(),
            )),
        }
    }
    pub fn cvm(&self) -> cvm::CVMBuilder {
        cvm::CVMBuilder::new(self.client.clone())
    }
}

#[derive(Debug)]
pub struct TencentCloudBaseClient {
    client: ClientWithMiddleware,
    ak: String,
    sk: String,
    base_url: String,
}

impl TencentCloudBaseClient {
    pub fn new(ak: String, sk: String) -> Self {
        let reqwest_client = reqwest::Client::new();
        let client = ClientBuilder::new(reqwest_client)
            .with(SignatureMiddleware)
            .build();
        Self {
            client,
            ak,
            sk,
            base_url: "https://tencentcloudapi.com".to_owned(),
        }
        // base_url: "https://cvm.tencentcloudapi.com".to_owned(),
    }

    pub fn get(&self, service: &str) -> RequestBuilder {
        self.client
            .get(build_service_api_url(&self.base_url, service))
            .with_extension(self.signature_context(service))
    }

    pub fn post(&self, service: &str) -> RequestBuilder {
        self.client
            .post(build_service_api_url(&self.base_url, service))
            .with_extension(self.signature_context(service))
    }

    pub fn signature_context(&self, service: &str) -> SignatureContext {
        SignatureContext {
            ak: self.ak.clone(),
            sk: self.sk.clone(),
            signed_headers: Some(vec![header::CONTENT_TYPE, header::HOST]),
            service_name: service.to_owned(),
        }
    }
}

fn build_service_api_url(url: &str, service_name: &str) -> String {
    format!(
        "https://{}.{}",
        service_name,
        &url[url.find("://").map_or(0, |x| x + 3)..]
    )
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[allow(unused_imports)]
    use super::*;

    #[tokio::test]
    async fn test_send() {
        let base_client = TencentCloudBaseClient::new("ak".into(), "sk".into());
        let r = base_client
            .get("cvm")
            .json(&json!({"key":"value"}))
            .send()
            .await
            .unwrap();
        println!("r: {r:?}");
    }
}