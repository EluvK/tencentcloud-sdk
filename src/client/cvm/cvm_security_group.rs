use std::sync::Arc;

use serde::Deserialize;
use serde_json::json;

use crate::{
    client::{TencentCloudBaseClient, ACTION_HEADER, REGION_HEADER},
    constant::Region,
};

pub struct SecurityGroupBuilder {
    client: Arc<TencentCloudBaseClient>,
    service_name: String,
}

const DESCRIBE_SECURITY_GROUPS: &str = "DescribeSecurityGroups";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeSecurityGroups {
    pub response: DescribeSecurityGroupsInner,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeSecurityGroupsInner {
    pub total_count: usize,
    pub security_group_set: Vec<SecurityGroupInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecurityGroupInfo {
    pub security_group_id: String,
    pub security_group_name: String,
    pub security_group_desc: String,
    pub project_id: String,
    pub is_default: bool,
    pub created_time: String,
}

impl SecurityGroupBuilder {
    pub fn new(client: Arc<TencentCloudBaseClient>) -> Self {
        Self {
            client,
            service_name: "vpc".into(),
        }
    }
    pub async fn describe_security_groups(
        &self,
        region: &Region,
    ) -> anyhow::Result<Vec<SecurityGroupInfo>> {
        let resp = self
            .client
            .post(&self.service_name)
            .header(ACTION_HEADER, DESCRIBE_SECURITY_GROUPS)
            .header(REGION_HEADER, region.to_string())
            .json(&json!({}))
            .send()
            .await?;
        let body: DescribeSecurityGroups = resp.json().await?;
        Ok(body.response.security_group_set)
    }
}
