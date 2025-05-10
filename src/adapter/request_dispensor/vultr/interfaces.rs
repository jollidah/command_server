use serde::{Deserialize, Serialize};

use crate::adapter::request_dispensor::vultr::VultrClient;
use crate::errors::ServiceError;
pub trait ExecuteVultrCommand: Serialize {
    async fn execute(self, vultr_client: &VultrClient) -> Result<impl Deserialize, ServiceError>;
}
