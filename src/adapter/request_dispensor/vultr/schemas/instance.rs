use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    adapter::request_dispensor::vultr::{
        interfaces::{
            ExecuteVultrCreateCommand, ExecuteVultrDeleteCommand, ExecuteVultrUpdateCommand,
        },
        VultrClient,
    },
    domain::project::enums::BackupStatus,
    errors::ServiceError,
};

use super::extract_schema_from_response;

#[derive(Serialize, Deserialize)]
pub struct ListCompute;
#[derive(Serialize, Deserialize)]
pub struct CreateCompute {
    pub region: String,
    pub plan: String,
    pub label: String,
    pub os_id: i64,
    pub backups: BackupStatus,
    pub hostname: String,
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

// pub struct VultrCreateCommand {
//     pub command_type: String,  // 명령어 타입
//     pub data: Value,  // 실제 데이터
// }

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
