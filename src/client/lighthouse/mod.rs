use std::sync::Arc;

use super::TencentCloudBaseClient;

pub mod lighthouse_firewall;

pub struct LighthouseBuilder {
    client: Arc<TencentCloudBaseClient>,
}

impl LighthouseBuilder {
    pub fn new(client: Arc<TencentCloudBaseClient>) -> Self {
        Self { client }
    }

    pub fn firewall(&self) -> lighthouse_firewall::LighthouseFirewallBuilder {
        lighthouse_firewall::LighthouseFirewallBuilder::new(self.client.clone())
    }
}
