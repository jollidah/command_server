use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    domain::project::diagrams::ObjectStorage,
    errors::ServiceError,
};

pub struct ObjectStorageCommandFactory;
impl ObjectStorageCommandFactory {
    pub fn list_object_storage() -> ListObjectStorage {
        ListObjectStorage {}
    }
    pub fn create_object_storage(
        cluster_id: i64,
        tier_id: i64,
        label: String,
    ) -> CreateObjectStorage {
        CreateObjectStorage {
            cluster_id,
            tier_id,
            label,
        }
    }
    pub fn get_object_storage(id: Uuid) -> GetObjectStorage {
        GetObjectStorage { id }
    }
    pub fn delete_object_storage(id: Uuid) -> DeleteObjectStorage {
        DeleteObjectStorage { id: Some(id) }
    }
    pub fn update_object_storage(id: Uuid, label: String) -> UpdateObjectStorage {
        UpdateObjectStorage {
            id: Some(id),
            label,
        }
    }
}

#[derive(Serialize)]
pub struct ListObjectStorage;

#[derive(Serialize, Deserialize)]
pub struct CreateObjectStorage {
    cluster_id: i64,
    tier_id: i64,
    label: String,
}

#[derive(Serialize)]
pub struct GetObjectStorage {
    id: Uuid, // Use id as path parameter
}

#[derive(Serialize, Deserialize)]
pub struct DeleteObjectStorage {
    // This id can be None if the id is not assigned yet
    pub id: Option<Uuid>,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateObjectStorage {
    #[serde(skip_serializing)]
    pub id: Option<Uuid>, // Use id as path parameter
    label: String,
}
impl GetObjectStorage {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
// #[allow(refining_impl_trait)]
// impl ExecuteVultrCommand for ListObjectStorage {
//     async fn execute<'a>(self, vultr_client: &'a VultrClient, id_store: &'a mut HashMap<i64, String>) -> Result<Vec<ObjectStorage>, ServiceError> {
//         let response = vultr_client
//             .build_request(Method::GET, "object-storage".to_string())
//             .send()
//             .await?;
//         extract_schema_from_response::<Vec<ObjectStorage>>(response, "object_storages").await
//     }
// }
#[allow(refining_impl_trait)]
impl ExecuteVultrCreateCommand for CreateObjectStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Value, ServiceError> {
        let response = vultr_client
            .build_request(Method::POST, "object-storage".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Value>(response, "object_storage").await
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrGetCommand for GetObjectStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<ObjectStorage, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, format!("object-storage/{}", self.id))
            .send()
            .await?;
        extract_schema_from_response::<ObjectStorage>(response, "object_storage").await
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrDeleteCommand for DeleteObjectStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        vultr_client
            .build_request(Method::DELETE, format!("object-storage/{}", id))
            .send()
            .await?;
        Ok(())
    }
}
#[allow(refining_impl_trait)]
impl ExecuteVultrUpdateCommand for UpdateObjectStorage {
    async fn execute(self, vultr_client: &VultrClient) -> Result<Option<Value>, ServiceError> {
        let id = self.id.ok_or_else(|| ServiceError::NotFound)?;
        let response = vultr_client
            .build_request(Method::PUT, format!("object-storage/{}", id))
            .send()
            .await?;
        Ok(None)
    }
    fn get_id(&self) -> Option<Uuid> {
        self.id
    }
}
