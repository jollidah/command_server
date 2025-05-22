use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use super::extract_schema_from_response;
use crate::{
    adapter::request_dispensor::vultr::{
        interfaces::{
            ExecuteVultrCreateCommand, ExecuteVultrDeleteCommand, ExecuteVultrGetCommand,
            ExecuteVultrUpdateCommand,
        },
        VultrClient,
    },
    domain::project::diagrams::BlockStorage,
    errors::ServiceError,
};

pub struct BlockStorageCommandFactory;
impl BlockStorageCommandFactory {
    pub fn list_block_storage() -> ListBlockStorage {
        ListBlockStorage {}
    }
    pub fn create_block_storage(region: String, size_gb: i64, label: String) -> CreateBlockStorage {
        CreateBlockStorage {
            region,
            size_gb,
            label,
        }
    }
    pub fn get_block_storage(id: Uuid) -> GetBlockStorage {
        GetBlockStorage { id }
    }
    pub fn delete_block_storage(id: Uuid) -> DeleteBlockStorage {
        DeleteBlockStorage { id: Some(id) }
    }
    pub fn update_block_storage(id: Uuid, label: String, size_gb: i64) -> UpdateBlockStorage {
        UpdateBlockStorage {
            id: Some(id),
            label,
            size_gb,
        }
    }
    pub fn attach_block_storage_to_instance(
        id: Uuid,
        instance_id: Uuid,
        live: bool,
    ) -> AttachBlockStorageToCompute {
        AttachBlockStorageToCompute {
            id: Some(id),
            instance_id,
            live,
        }
    }
    pub fn detach_block_storage_from_instance(
        id: Uuid,
        live: bool,
    ) -> DetachBlockStorageFromCompute {
        DetachBlockStorageFromCompute { id: Some(id), live }
    }
}

#[derive(Serialize)]
pub struct ListBlockStorage;

#[derive(Serialize, Deserialize)]
pub struct CreateBlockStorage {
    region: String,
    size_gb: i64, // New size of the Block Storage in GB. Size may range between 10 and 40000 depending on the block_type
    label: String,
}

#[derive(Serialize)]
pub struct GetBlockStorage {
    id: Uuid,
}

impl GetBlockStorage {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteBlockStorage {
    // This id can be None if the id is not assigned yet
    pub id: Option<Uuid>,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateBlockStorage {
    #[serde(skip_serializing)]
    pub id: Option<Uuid>,
    label: String,
    size_gb: i64, // New size of the Block Storage in GB. Size may range between 10 and 40000 depending on the block_type
}
#[derive(Serialize, Deserialize)]
pub struct AttachBlockStorageToCompute {
    #[serde(skip_serializing)]
    pub id: Option<Uuid>,
    instance_id: Uuid,
    live: bool, // true: do not restart the instance
}

#[derive(Serialize, Deserialize)]
pub struct DetachBlockStorageFromCompute {
    #[serde(skip_serializing)]
    pub id: Option<Uuid>,
    live: bool, // true: do not restart the instance
}
// #[allow(refining_impl_trait)]
// impl ExecuteVultrCommand for ListBlockStorage {
//     async fn execute<'a>(self, vultr_client: &'a VultrClient, id_store: &'a mut HashMap<i64, String>) -> Result<Vec<BlockStorage>, ServiceError> {
//         let response = vultr_client
//             .build_request(Method::GET, "blocks".to_string())
//             .send()
//             .await?;
//         extract_schema_from_response::<Vec<BlockStorage>>(response, "blocks").await
//     }
// }
#[allow(refining_impl_trait)]
impl ExecuteVultrCreateCommand for CreateBlockStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Value, ServiceError> {
        let response = vultr_client
            .build_request(Method::POST, "blocks".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Value>(response, "block").await
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrGetCommand for GetBlockStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<BlockStorage, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, format!("blocks/{}", self.id))
            .send()
            .await?;
        extract_schema_from_response::<BlockStorage>(response, "block").await
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrDeleteCommand for DeleteBlockStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        vultr_client
            .build_request(Method::DELETE, format!("blocks/{}", id))
            .send()
            .await?;
        Ok(())
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrUpdateCommand for UpdateBlockStorage {
    fn get_id(&self) -> Option<Uuid> {
        self.id
    }
    async fn execute(self, vultr_client: &VultrClient) -> Result<Option<Value>, ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        let response = vultr_client
            .build_request(Method::PUT, format!("blocks/{}", id))
            .send()
            .await?;
        Ok(None)
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrUpdateCommand for AttachBlockStorageToCompute {
    fn get_id(&self) -> Option<Uuid> {
        self.id
    }
    async fn execute(self, vultr_client: &VultrClient) -> Result<Option<Value>, ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        vultr_client
            .build_request(Method::POST, format!("blocks/{}/attach", id))
            .send()
            .await?;
        Ok(None)
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrUpdateCommand for DetachBlockStorageFromCompute {
    fn get_id(&self) -> Option<Uuid> {
        self.id
    }
    async fn execute(self, vultr_client: &VultrClient) -> Result<Option<Value>, ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        vultr_client
            .build_request(Method::POST, format!("blocks/{}/detach", id))
            .send()
            .await?;
        Ok(None)
    }
}
