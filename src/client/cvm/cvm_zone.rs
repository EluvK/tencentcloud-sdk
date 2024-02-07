use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::{
    client::constant::{ACTION_HEADER, REGION_HEADER},
    constant::Region,
};

const DESCRIBE_ZONES: &str = "DescribeZones";

use super::*;
pub struct CVMZoneBuilder {
    client: Arc<TencentCloudBaseClient>,
    service_name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeZoneResponse {
    pub response: DescribeZoneResponseInner,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeZoneResponseInner {
    #[allow(unused)]
    pub total_count: usize,
    pub zone_set: Vec<ZoneInfo>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ZoneInfo {
    pub zone: String,
}
impl CVMZoneBuilder {
    pub fn new(client: Arc<TencentCloudBaseClient>) -> Self {
        Self {
            client,
            service_name: "cvm".into(),
            version: "2017-03-12".into(),
        }
    }
    pub async fn describe_zone(&self, region: &Region) -> anyhow::Result<Option<Vec<String>>> {
        let resp = self
            .client
            .post(&self.service_name, &self.version)
            .header(ACTION_HEADER, DESCRIBE_ZONES)
            .header(REGION_HEADER, region.to_string())
            .json(&json!({}))
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => {
                let body: DescribeZoneResponse = resp.json().await?;
                // println!("DescribeZoneResponse body: {body:?}");
                Ok(Some(
                    body.response.zone_set.into_iter().map(|z| z.zone).collect(),
                ))
            }
            rest => Err(anyhow::anyhow!(
                "err get code {rest}, msg {}",
                resp.text().await?
            )),
        }
    }
}
