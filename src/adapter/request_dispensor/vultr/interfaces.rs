use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::adapter::request_dispensor::vultr::VultrClient;
use crate::errors::ServiceError;

pub trait ExecuteVultrCreateCommand: Serialize {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Value, ServiceError>;
}

pub trait ExecuteVultrUpdateCommand: Serialize {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Option<Value>, ServiceError>;
}

pub trait ExecuteVultrDeleteCommand: Serialize {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError>;
}

pub trait ExecuteVultrGetCommand: Serialize {
    async fn execute(
        self,
        vultr_client: &VultrClient,
    ) -> Result<impl DeserializeOwned, ServiceError>;
}
