use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::extract_schema_from_response;
use crate::{
    adapter::request_dispensor::vultr::{interfaces::ExecuteVultrCommand, VultrClient},
    errors::ServiceError,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockStorageType {
    HighPerformance,
    LowPerformance,
}

#[derive(Serialize, Deserialize)]
pub struct BlockStorage {
    id: Uuid,
    date_created: DateTime<Utc>,
    cost: i64,
    status: String,
    size_gb: i64,
    region: String,
    attached_to_instance: Uuid,
    label: String,
    mount_id: String,
    block_type: BlockStorageType, // TODO validate size_gb based on block_type
}

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
    pub fn get_block_storage(block_id: Uuid) -> GetBlockStorage {
        GetBlockStorage { block_id }
    }
    pub fn delete_block_storage(block_id: Uuid) -> DeleteBlockStorage {
        DeleteBlockStorage { block_id }
    }
    pub fn update_block_storage(block_id: Uuid, label: String, size_gb: i64) -> UpdateBlockStorage {
        UpdateBlockStorage {
            block_id,
            label,
            size_gb,
        }
    }
    pub fn attach_block_storage_to_instance(
        block_id: Uuid,
        instance_id: Uuid,
        live: bool,
    ) -> AttachBlockStorageToInstance {
        AttachBlockStorageToInstance {
            block_id,
            instance_id,
            live,
        }
    }
    pub fn detach_block_storage_from_instance(
        block_id: Uuid,
        live: bool,
    ) -> DetachBlockStorageFromInstance {
        DetachBlockStorageFromInstance { block_id, live }
    }
}

#[derive(Serialize)]
pub struct ListBlockStorage;

#[derive(Serialize)]
pub struct CreateBlockStorage {
    region: String,
    size_gb: i64, // New size of the Block Storage in GB. Size may range between 10 and 40000 depending on the block_type
    label: String,
}

#[derive(Serialize)]
pub struct GetBlockStorage {
    block_id: Uuid,
}

#[derive(Serialize)]
pub struct DeleteBlockStorage {
    block_id: Uuid,
}
#[derive(Serialize)]
pub struct UpdateBlockStorage {
    #[serde(skip_serializing)]
    block_id: Uuid,
    label: String,
    size_gb: i64, // New size of the Block Storage in GB. Size may range between 10 and 40000 depending on the block_type
}
#[derive(Serialize)]
pub struct AttachBlockStorageToInstance {
    #[serde(skip_serializing)]
    block_id: Uuid,
    instance_id: Uuid,
    live: bool, // true: do not restart the instance
}

#[derive(Serialize)]
pub struct DetachBlockStorageFromInstance {
    #[serde(skip_serializing)]
    block_id: Uuid,
    live: bool, // true: do not restart the instance
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for ListBlockStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Vec<BlockStorage>, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, "blocks".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Vec<BlockStorage>>(response, "blocks").await
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for CreateBlockStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<BlockStorage, ServiceError> {
        let response = vultr_client
            .build_request(Method::POST, "blocks".to_string())
            .send()
            .await?;
        extract_schema_from_response::<BlockStorage>(response, "block").await
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for GetBlockStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<BlockStorage, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, format!("blocks/{}", self.block_id))
            .send()
            .await?;
        extract_schema_from_response::<BlockStorage>(response, "block").await
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for DeleteBlockStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(Method::DELETE, format!("blocks/{}", self.block_id))
            .send()
            .await?;
        Ok(())
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for UpdateBlockStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(Method::PUT, format!("blocks/{}", self.block_id))
            .send()
            .await?;
        Ok(())
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for AttachBlockStorageToInstance {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(Method::POST, format!("blocks/{}/attach", self.block_id))
            .send()
            .await?;
        Ok(())
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for DetachBlockStorageFromInstance {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(Method::POST, format!("blocks/{}/detach", self.block_id))
            .send()
            .await?;
        Ok(())
    }
}
