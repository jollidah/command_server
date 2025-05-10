use crate::errors::ServiceError;

impl From<rocksdb::Error> for ServiceError {
    fn from(error: rocksdb::Error) -> Self {
        ServiceError::KVStoreError(Box::new(error))
    }
}
