use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    adapter::request_dispensor::vultr::{
        self, get_vultr_client,
        interfaces::{
            ExecuteVultrCreateCommand, ExecuteVultrDeleteCommand, ExecuteVultrUpdateCommand,
        },
        VultrClient,
    },
    domain::project::enums::BackupStatus,
    errors::ServiceError,
};

use super::extract_schema_from_response;

pub struct ComputeCommandFactory;
impl ComputeCommandFactory {
    pub fn list_instance() -> ListCompute {
        ListCompute {}
    }
    pub fn create_instance(
        region: String,
        plan: String,
        label: String,
        os_id: i64,
        backups: BackupStatus,
        hostname: String,
    ) -> CreateCompute {
        CreateCompute {
            region,
            plan,
            label,
            os_id,
            backups,
            hostname,
        }
    }
    pub fn get_instance(id: Uuid) -> GetCompute {
        GetCompute { id }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn update_instance(
        id: Uuid,
        backups: BackupStatus,
        firewall_group_id: String,
        os_id: i64,
        plan: String,
        ddos_protection: bool,
        label: String,
    ) -> UpdateCompute {
        UpdateCompute {
            id: Some(id),
            backups,
            firewall_group_id,
            os_id,
            plan,
            ddos_protection,
            label,
        }
    }
    pub fn delete_instance(id: Uuid) -> DeleteCompute {
        DeleteCompute { id: Some(id) }
    }
}
#[derive(Serialize, Deserialize)]
pub struct ListCompute;
#[derive(Serialize, Deserialize)]
pub struct CreateCompute {
    region: String,
    plan: String,
    label: String,
    os_id: i64,
    backups: BackupStatus,
    hostname: String,
}
#[derive(Serialize, Deserialize)]
pub struct GetCompute {
    id: Uuid,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateCompute {
    #[serde(skip_serializing)]
    pub id: Option<Uuid>, // Use id as path parameter
    backups: BackupStatus,
    firewall_group_id: String,
    os_id: i64,
    plan: String,
    ddos_protection: bool,
    label: String,
}
#[derive(Serialize, Deserialize)]
pub struct DeleteCompute {
    // This id can be None if the id is not assigned yet
    pub id: Option<Uuid>,
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCreateCommand for CreateCompute {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Value, ServiceError> {
        let response = vultr_client
            .build_request(Method::POST, "instances".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Value>(response, "instance").await
    }
}

// #[allow(refining_impl_trait)]
// impl ExecuteVultrGetCommand for GetCompute {
//     async fn execute<'a>(self, vultr_client: &'a VultrClient, id_store: &'a mut HashMap<i64, String>) -> Result<(), ServiceError> {
//         let response = vultr_client
//             .build_request(Method::GET, format!("instances/{}", self.id))
//             .send()
//             .await?;
//         extract_schema_from_response::<Compute>(response, "instance").await?;
//         Ok(())
//     }
// }

#[allow(refining_impl_trait)]
impl ExecuteVultrUpdateCommand for UpdateCompute {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Option<Value>, ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        let response = vultr_client
            .build_request(Method::PUT, format!("instances/{}", id))
            .send()
            .await?;
        Ok(Some(
            extract_schema_from_response::<Value>(response, "instance").await?,
        ))
    }

    fn get_id(&self) -> Option<Uuid> {
        self.id
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrDeleteCommand for DeleteCompute {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        vultr_client
            .build_request(Method::DELETE, format!("instances/{}", id))
            .send()
            .await?;
        Ok(())
    }
}
