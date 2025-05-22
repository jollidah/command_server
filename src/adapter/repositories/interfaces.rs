use sqlx::PgConnection;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::errors::ServiceError;

pub trait TExecutor {
    fn new() -> Arc<RwLock<Self>>;
    fn transaction(&mut self) -> &mut PgConnection;
    fn begin(&mut self) -> impl std::future::Future<Output = Result<(), ServiceError>>;
    fn commit(&mut self) -> impl std::future::Future<Output = Result<(), ServiceError>>;
    fn close(&mut self) -> impl std::future::Future<Output = ()>;
    fn rollback(&mut self) -> impl std::future::Future<Output = Result<(), ServiceError>>;
}
