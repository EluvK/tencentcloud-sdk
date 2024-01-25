use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::{
    client::constant::{ACTION_HEADER, REGION_HEADER},
    constant::{InstanceType, Region},
};

use super::*;

pub struct CVMInstanceBuilder {
    client: Arc<TencentCloudBaseClient>,
    service_name: String,
}

/// InquiryPriceRunInstancesResponse
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InquiryPriceRunInstancesResponse {
    pub response: InquiryPriceRunInstancesResponseInner,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InquiryPriceRunInstancesResponseInner {
    pub price: Price,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Price {
    pub instance_price: PriceDetail,
    pub bandwidth_price: PriceDetail,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PriceDetail {
    pub unit_price: f64,
    pub unit_price_discount: f64,
    pub charge_unit: String,
    pub discount: f64,
}

/// RunInstancesResponse
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RunInstancesResponse {
    pub response: RunInstancesResponseInner,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RunInstancesResponseInner {
    pub instance_id_set: Vec<String>,
}

const DESCRIBE_INSTANCES: &str = "DescribeInstances";
const INQUIRY_PRICE_RUN_INSTANCES: &str = "InquiryPriceRunInstances";
const RUN_INSTANCES: &str = "RunInstances";
const TERMINATE_INSTANCES: &str = "TerminateInstances";

impl CVMInstanceBuilder {
    pub fn new(client: Arc<TencentCloudBaseClient>) -> Self {
        Self {
            client,
            service_name: "cvm".into(),
        }
    }
    pub async fn describe_instance(&self, region: &Region) -> anyhow::Result<()> {
        let resp = self
            .client
            .post(&self.service_name)
            .header(ACTION_HEADER, DESCRIBE_INSTANCES)
            .header(REGION_HEADER, region.to_string())
            .json(&json!({}))
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => {
                let body = resp.text().await;
                // println!("body: {body:?}");
                Ok(())
            }
            rest => Err(anyhow::anyhow!(
                "err get code {rest}, msg {}",
                resp.text().await?
            )),
        }
    }

    /// set default SPOTPAID/Ubuntu2204/20GB disk
    pub async fn query_price(
        &self,
        region: &Region,
        zone: &str,
        instance_type: &InstanceType,
    ) -> anyhow::Result<Price> {
        let resp = self
            .client
            .post(&self.service_name)
            .header(ACTION_HEADER, INQUIRY_PRICE_RUN_INSTANCES)
            .header(REGION_HEADER, region.to_string())
            .json(&json!({
                "InstanceChargeType": "SPOTPAID",
                "ImageId": "img-487zeit5",
                "Placement": {
                    "Zone": zone
                },
                "InstanceType": instance_type.to_string(),
                "InstanceCount": 1,
                "SystemDisk": {
                    "DiskSize": 20,
                },
                "InternetAccessible": {
                    "InternetChargeType": "TRAFFIC_POSTPAID_BY_HOUR",
                    "InternetMaxBandwidthOut": 10,
                    "PublicIpAssigned": true
                }
            }))
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => {
                let body: InquiryPriceRunInstancesResponse = resp.json().await?;
                // println!("body: {body:?}");
                Ok(body.response.price)
            }
            rest => Err(anyhow::anyhow!(
                "err get code {rest}, msg {}",
                resp.text().await?
            )),
        }
    }

    /// set default SPOTPAID/Ubuntu2204/20GB disk
    pub async fn run_instance(
        &self,
        region: &Region,
        zone: &str,
        instance_type: &InstanceType,
        key_ids: Vec<String>,
    ) -> anyhow::Result<String> {
        let mut body = json!({
            "InstanceChargeType": "SPOTPAID",
            "ImageId": "img-487zeit5",
            "Placement": {
                "Zone": zone
            },
            "InstanceType": instance_type.to_string(),
            "InstanceCount": 1,
            "SystemDisk": {
                "DiskSize": 20,
            },
            "InternetAccessible": {
                "InternetChargeType": "TRAFFIC_POSTPAID_BY_HOUR",
                "InternetMaxBandwidthOut": 10,
                "PublicIpAssigned": true
            }
        });
        if !key_ids.is_empty() {
            body["LoginSettings"] = json!({
                "KeyIds": key_ids
            });
        }
        let resp = self
            .client
            .post(&self.service_name)
            .header(ACTION_HEADER, RUN_INSTANCES)
            .header(REGION_HEADER, region.to_string())
            .json(&body)
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => {
                let body: RunInstancesResponse = resp.json().await?;
                println!("body: {body:?}");
                body.response
                    .instance_id_set
                    .into_iter()
                    .nth(0)
                    .ok_or(anyhow::anyhow!("panic!! response missing id ???"))
            }
            rest => Err(anyhow::anyhow!(
                "err get code {rest}, msg {}",
                resp.text().await?
            )),
        }
    }

    pub async fn terminate_instance(
        &self,
        region: &Region,
        instance_id: &str,
    ) -> anyhow::Result<()> {
        let resp = self
            .client
            .post(&self.service_name)
            .header(ACTION_HEADER, TERMINATE_INSTANCES)
            .header(REGION_HEADER, region.to_string())
            .json(&json!({
                "InstanceIds": [instance_id]
            }))
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => {
                let body = resp.text().await;
                println!("body: {body:?}");
                Ok(())
            }
            rest => Err(anyhow::anyhow!(
                "err get code {rest}, msg {}",
                resp.text().await?
            )),
        }
    }
}