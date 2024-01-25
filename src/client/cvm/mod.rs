use std::sync::Arc;

use super::TencentCloudBaseClient;

pub mod cvm_instance;
pub mod cvm_key;
pub mod cvm_zone;

pub struct CVMBuilder {
    client: Arc<TencentCloudBaseClient>,
}

impl CVMBuilder {
    pub fn new(client: Arc<TencentCloudBaseClient>) -> Self {
        Self { client }
    }

    pub fn instances(&self) -> cvm_instance::CVMInstanceBuilder {
        cvm_instance::CVMInstanceBuilder::new(self.client.clone())
    }

    pub fn zone(&self) -> cvm_zone::CVMZoneBuilder {
        cvm_zone::CVMZoneBuilder::new(self.client.clone())
    }

    pub fn keys(&self) -> cvm_key::CVMKeyBuilder {
        cvm_key::CVMKeyBuilder::new(self.client.clone())
    }
}