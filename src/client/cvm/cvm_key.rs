use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::{
    client::{ACTION_HEADER, REGION_HEADER},
    constant::Region,
};

use super::*;
pub struct CVMKeyBuilder {
    client: Arc<TencentCloudBaseClient>,
    service_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeKeyPairsReponse {
    pub response: DescribeKeyPairsResponseInner,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeKeyPairsResponseInner {
    #[allow(unused)]
    pub total_count: usize,
    pub key_pair_set: Vec<KeyPair>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyPair {
    pub key_id: String,
    pub key_name: String,
    pub public_key: String,
    pub created_time: String,
}

const DESCRIBE_KEY_PAIRS: &str = "DescribeKeyPairs";

impl CVMKeyBuilder {
    pub fn new(client: Arc<TencentCloudBaseClient>) -> Self {
        Self {
            client,
            service_name: "cvm".into(),
        }
    }
    pub async fn describe_key_pairs(&self, region: &Region) -> anyhow::Result<Vec<KeyPair>> {
        let resp = self
            .client
            .post(&self.service_name)
            .header(ACTION_HEADER, DESCRIBE_KEY_PAIRS)
            .header(REGION_HEADER, region.to_string())
            .json(&json!({}))
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => {
                let body: DescribeKeyPairsReponse = resp.json().await?;
                // println!("DescribeZoneResponse body: {body:?}");
                Ok(body.response.key_pair_set)
            }
            rest => Err(anyhow::anyhow!(
                "err get code {rest}, msg {}",
                resp.text().await?
            )),
        }
    }
}