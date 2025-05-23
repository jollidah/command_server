use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::extract_schema_from_response;
use crate::{
    adapter::request_dispensor::vultr::{
        interfaces::{
            ExecuteVultrCreateCommand, ExecuteVultrDeleteCommand, ExecuteVultrUpdateCommand,
        },
        VultrClient,
    },
    domain::project::enums::DatabaseEngine,
    errors::ServiceError,
};
#[derive(Serialize)]
pub struct ListManagedDatabasePlans;
#[derive(Serialize)]
pub struct ListManagedDatabase;
#[derive(Serialize, Deserialize)]
pub struct CreateManagedDatabase {
    database_engine: DatabaseEngine,
    database_engine_version: i64,
    region: String,
    plan: String,
    label: String,
}
#[derive(Serialize)]
pub struct GetManagedDatabase {
    pub id: Uuid, // Use id as path parameter
}
#[derive(Serialize, Deserialize)]
pub struct UpdateManagedDatabase {
    #[serde(skip_serializing)]
    pub id: Option<Uuid>, // Use id as path parameter
    plan: String,
    label: String,
}
#[derive(Serialize, Deserialize)]
pub struct DeleteManagedDatabase {
    // This id can be None if the id is not assigned yet
    pub id: Option<Uuid>,
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCreateCommand for CreateManagedDatabase {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Value, ServiceError> {
        let response = vultr_client
            .build_request(Method::POST, "databases".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Value>(response, "database").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrUpdateCommand for UpdateManagedDatabase {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Option<Value>, ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        let response = vultr_client
            .build_request(Method::PUT, format!("databases/{}", id))
            .send()
            .await?;
        Ok(Some(
            extract_schema_from_response::<Value>(response, "database").await?,
        ))
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrDeleteCommand for DeleteManagedDatabase {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        vultr_client
            .build_request(Method::DELETE, format!("databases/{}", id))
            .send()
            .await?;
        Ok(())
    }
}
