use rocksdb::DB;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

use crate::{config::get_config, errors::ServiceError};

use super::interfaces::KVStore;

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
            .ok_or_else(|| ServiceError::KVStoreError(Box::new("Value not found")))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::auth::VerificationCode;
    use uuid::Uuid;

    async fn tear_down() {
        let rocks_db = get_rocks_db().await;
        let db = rocks_db.db.lock().await;

        // Delete all key-value pairs using iterator
        let mut iter = db.iterator(rocksdb::IteratorMode::Start);
        while let Some(item) = iter.next() {
            let (key, _) = item.unwrap();
            db.delete(key.as_ref()).expect("Failed to delete key");
        }
    }

    #[tokio::test]
    async fn test_set() {
        // GIVEN
        tear_down().await;

        let rocks_db = get_rocks_db().await;
        let binding = Uuid::new_v4();
        let key = binding.as_bytes();
        let verification_code = VerificationCode::new();

        // WHEN
        rocks_db
            .insert(key, &verification_code.to_bytes().unwrap())
            .await
            .unwrap();

        // THEN
        let db = rocks_db.db.lock().await;
        let mut iter = db.iterator(rocksdb::IteratorMode::Start);
        let mut found = false;
        while let Some(item) = iter.next() {
            let (k, v) = item.unwrap();
            if k.as_ref() == key {
                found = true;
                assert_eq!(VerificationCode::from_bytes(&v).unwrap(), verification_code);
            } else {
                panic!("Unknown key found in database");
            }
        }
        assert!(found, "Key not found in database");
    }

    #[tokio::test]
    async fn test_get() {
        // GIVEN
        tear_down().await;

        let rocks_db = get_rocks_db().await;
        let binding = Uuid::new_v4();
        let key = binding.as_bytes();
        let verification_code = VerificationCode::new();

        let binding = Uuid::new_v4();
        let unknown_key = binding.as_bytes();

        // WHEN
        rocks_db
            .insert(key, &verification_code.to_bytes().unwrap())
            .await
            .unwrap();

        // THEN
        assert_eq!(
            rocks_db.get(key).await.unwrap(),
            verification_code.to_bytes().unwrap()
        );
        assert!(matches!(
            rocks_db.get(unknown_key).await.unwrap_err(),
            ServiceError::KVStoreError(_)
        ));
    }

    #[tokio::test]
    async fn test_delete() {
        // GIVEN
        tear_down().await;

        let rocks_db = get_rocks_db().await;
        let binding = Uuid::new_v4();
        let key = binding.as_bytes();
        let verification_code = VerificationCode::new();

        // WHEN
        rocks_db
            .insert(key, &verification_code.to_bytes().unwrap())
            .await
            .unwrap();
        assert_eq!(
            rocks_db.get(key).await.unwrap(),
            verification_code.to_bytes().unwrap()
        );

        // THEN
        rocks_db.delete(key).await.unwrap();
        assert!(matches!(
            rocks_db.get(key).await.unwrap_err(),
            ServiceError::KVStoreError(_)
        ));
    }
}
