use crate::errors::ServiceError;
use sqlx::Error as SqlxError;

impl From<SqlxError> for ServiceError {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::Database(err) => ServiceError::DatabaseConnectionError(Box::new(err)),
            _ => ServiceError::NotFound,
        }
    }
}
