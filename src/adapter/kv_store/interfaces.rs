use uuid::Uuid;

use crate::{domain::auth::private_key::PublicKey, errors::ServiceError};

// TODO: Add a cleanup method to the trait
pub(crate) trait KVStore {
    async fn insert(&self, key: &[u8], value: &[u8]) -> Result<(), ServiceError>;
    async fn pop(&self, key: &[u8]) -> Result<Vec<u8>, ServiceError>;
    async fn get(&self, key: &[u8]) -> Result<Vec<u8>, ServiceError>;
    async fn delete(&self, key: &[u8]) -> Result<(), ServiceError>;
}

pub(crate) trait VultrKeyPairStore: KVStore {
    const PRIVATE_KEY_NAME: &'static [u8] = b"private_key";
    const PUBLIC_KEY_NAME: &'static [u8] = b"public_key";
    async fn get_or_create_public_key(&self) -> Result<PublicKey, ServiceError>;
}

#[allow(unused)]
pub(crate) trait SessionStore: KVStore {
    const LOCK_NAME: &'static str = "lock";
    fn extract_user_ids_from_session(value: Option<Vec<u8>>) -> Result<Vec<String>, ServiceError>;
    async fn add_user_to_session(
        &self,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ServiceError>;
    async fn remove_user_from_session(
        &self,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ServiceError>;
    async fn get_user_ids_from_session(&self, project_id: Uuid) -> Result<Vec<Uuid>, ServiceError>;
}
