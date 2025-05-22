use rocksdb::DB;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

use crate::{
    config::get_config,
    domain::auth::private_key::{PublicKey, VultrKeyPair},
    errors::ServiceError,
};

use super::interfaces::{KVStore, VultrKeyPairStore};

pub(crate) struct RocksDB {
    pub(crate) db: Mutex<DB>,
}

impl RocksDB {
    pub fn new() -> Self {
        let mut options = rocksdb::Options::default();
        let config = get_config();

        options.set_write_buffer_size(config.rocksdb_buffer_size);
        options.set_max_write_buffer_number(2);
        options.create_if_missing(true);

        let db = DB::open(&options, &config.rocksdb_path).expect("Failed to open RocksDB");

        RocksDB { db: Mutex::new(db) }
    }
}

pub async fn get_rocks_db() -> Arc<RocksDB> {
    static DB_INSTANCE: LazyLock<Arc<RocksDB>> = LazyLock::new(|| Arc::new(RocksDB::new()));
    DB_INSTANCE.clone()
}

impl KVStore for RocksDB {
    async fn insert(&self, key: &[u8], value: &[u8]) -> Result<(), ServiceError> {
        let db = self.db.lock().await;
        db.put(key, value)
            .map_err(|err| ServiceError::KVStoreError(Box::new(err)))
    }

    async fn get(&self, key: &[u8]) -> Result<Vec<u8>, ServiceError> {
        let db = self.db.lock().await;
        db.get(key)
            .map_err(|err| ServiceError::KVStoreError(Box::new(err)))?
            .ok_or(ServiceError::NotFound)
    }

    async fn pop(&self, key: &[u8]) -> Result<Vec<u8>, ServiceError> {
        let value = self.get(key).await?;
        self.delete(key).await?;
        Ok(value)
    }

    async fn delete(&self, key: &[u8]) -> Result<(), ServiceError> {
        let db = self.db.lock().await;
        db.delete(key)
            .map_err(|err| ServiceError::KVStoreError(Box::new(err)))
    }
}

impl VultrKeyPairStore for RocksDB {
    async fn get_or_create_public_key(&self) -> Result<PublicKey, ServiceError> {
        match self.get(Self::PUBLIC_KEY_NAME).await {
            Ok(value) => Ok(PublicKey::from_pem(&value)?),
            Err(_) => {
                let (public_key, private_key) = VultrKeyPair::generate_key_pair()?;
                self.insert(Self::PUBLIC_KEY_NAME, &public_key.key.public_key_to_pem()?)
                    .await?;
                self.insert(
                    Self::PRIVATE_KEY_NAME,
                    &private_key.key.private_key_to_pem()?,
                )
                .await?;
                Ok(public_key)
            }
        }
    }
}
