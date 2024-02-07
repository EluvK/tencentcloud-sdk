use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    client::{ACTION_HEADER, REGION_HEADER},
    constant::Region,
};

use super::*;

pub struct LighthouseFirewallBuilder {
    client: Arc<TencentCloudBaseClient>,
    service_name: String,
    version: String,
}

const DESCRIBE_FIREWALL_RULES_ACTION: &str = "DescribeFirewallRules";
const MODIFY_FIREWALL_RULES_ACTION: &str = "ModifyFirewallRules";

/// DescribeFirewallRulesResponse
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeFirewallRulesResponse {
    pub response: DescribeFirewallRulesResponseInner,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeFirewallRulesResponseInner {
    pub firewall_rule_set: Vec<FirewallRule>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirewallRule {
    pub protocol: String,
    pub port: String,
    pub cidr_block: String,
    pub action: String,
    pub firewall_rule_description: String,
}

impl LighthouseFirewallBuilder {
    pub fn new(client: Arc<TencentCloudBaseClient>) -> Self {
        Self {
            client,
            service_name: "lighthouse".into(),
            version: "2020-03-24".into(),
        }
    }

    pub async fn describe_firewall_rules(
        &self,
        region: &Region,
        instance_id: &str,
    ) -> anyhow::Result<DescribeFirewallRulesResponse> {
        let resp = self
            .client
            .post(&self.service_name, &self.version)
            .header(ACTION_HEADER, DESCRIBE_FIREWALL_RULES_ACTION)
            .header(REGION_HEADER, region.to_string())
            .json(&json!(
                {
                    "InstanceId": instance_id,
                }
            ))
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            rest => Err(anyhow::anyhow!(
                "describe_firewall_rules failed with status: {}",
                rest
            )),
        }
    }

    pub async fn modify_firewall_rules(
        &self,
        region: &Region,
        instance_id: &str,
        rules: Vec<FirewallRule>,
    ) -> anyhow::Result<()> {
        let resp = self
            .client
            .post(&self.service_name, &self.version)
            .header(ACTION_HEADER, MODIFY_FIREWALL_RULES_ACTION)
            .header(REGION_HEADER, region.to_string())
            .json(&json!(
                {
                    "InstanceId": instance_id,
                    "FirewallRules": rules,
                }
            ))
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(()),
            rest => Err(anyhow::anyhow!(
                "modify_firewall_rules failed with status: {}",
                rest
            )),
        }
    }
}
