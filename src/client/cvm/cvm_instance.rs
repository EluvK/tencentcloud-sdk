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

/// DescribeInstancesResponse
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeInstancesResponse {
    pub response: DescribeInstancesResponseInner,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeInstancesResponseInner {
    pub instance_set: Vec<Instance>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Instance {
    pub instance_state: InstanceState,
    pub public_ip_addresses: Option<Vec<String>>,
    pub instance_id: String,
    // todo more
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstanceState {
    PENDING, //表示创建中
    #[allow(non_camel_case_types)]
    LAUNCH_FAILED, //表示创建失败
    RUNNING, //表示运行中
    STOPPED, //表示关机
    STARTING, //表示开机中
    STOPPING, //表示关机中
    REBOOTING, //表示重启中
    SHUTDOWN, //表示停止待销毁
    TERMINATING, //表示销毁中。
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
    pub async fn describe_instance(
        &self,
        region: &Region,
    ) -> anyhow::Result<DescribeInstancesResponse> {
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
                Ok(resp.json().await?)
                // let body = resp.text().await;
                // println!("body: {body:?}");
                // Ok(())
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
        security_group: Vec<String>,
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
        if !security_group.is_empty() {
            body["SecurityGroupIds"] = security_group.into();
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
                // println!("body: {body:?}");
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
