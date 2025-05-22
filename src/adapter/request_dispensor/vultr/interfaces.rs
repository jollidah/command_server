use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::adapter::request_dispensor::vultr::VultrClient;
use crate::errors::ServiceError;

pub trait ExecuteVultrCreateCommand: Serialize {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Value, ServiceError>;
}

pub trait ExecuteVultrUpdateCommand: Serialize {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Option<Value>, ServiceError>;
    fn get_id(&self) -> Option<Uuid>;
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
