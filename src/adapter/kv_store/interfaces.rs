use crate::errors::ServiceError;

// TODO: Add a cleanup method to the trait
pub(crate) trait KVStore {
    async fn insert(&self, key: &[u8], value: &[u8]) -> Result<(), ServiceError>;
    async fn pop(&self, key: &[u8]) -> Result<Vec<u8>, ServiceError>;
    async fn get(&self, key: &[u8]) -> Result<Vec<u8>, ServiceError>;
    async fn delete(&self, key: &[u8]) -> Result<(), ServiceError>;
}
