use std::sync::Arc;

use crate::{config::get_config, errors::ServiceError};
use interfaces::TExecutor;
use sqlx::{PgConnection, Pool, Postgres, Transaction};
use tokio::sync::RwLock;
pub mod auth;
pub mod interfaces;
pub mod project;

pub struct SqlExecutor {
    pool: &'static Pool<Postgres>,
    transaction: Option<Transaction<'static, Postgres>>,
}

impl TExecutor for SqlExecutor {
    fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            pool: connection_pool(),
            transaction: None,
        }))
    }
    fn transaction(&mut self) -> &mut PgConnection {
        match self.transaction.as_mut() {
            Some(trx) => trx,
            None => panic!("Transaction Has Not Begun!"),
        }
    }
    async fn begin(&mut self) -> Result<(), ServiceError> {
        match self.transaction.as_mut() {
            None => {
                self.transaction = Some(
                    self.pool
                        .begin()
                        .await
                        .map_err(|err| ServiceError::DatabaseConnectionError(Box::new(err)))?,
                );
                Ok(())
            }
            Some(_trx) => {
                tracing::error!("Transaction Begun Already!");
                Err(ServiceError::DatabaseConnectionError(Box::new(
                    "Duplicate Transaction!",
                )))
            }
        }
    }

    async fn commit(&mut self) -> Result<(), ServiceError> {
        match self.transaction.take() {
            None => panic!("Transaction lost!"),
            Some(trx) => trx
                .commit()
                .await
                .map_err(|err| ServiceError::DatabaseConnectionError(Box::new(err))),
        }
    }

    async fn close(&mut self) {
        match self.transaction.take() {
            None => (),
            Some(trx) => {
                let _ = trx.rollback().await;
            }
        }
    }
}

static INIT: std::sync::OnceLock<sqlx::PgPool> = std::sync::OnceLock::new();
pub fn connection_pool() -> &'static sqlx::PgPool {
    INIT.get_or_init(|| {
        let url = get_config().database_url.clone();
        tracing::info!("Connecting to database with URL: {}", url);

        let opts: sqlx::postgres::PgConnectOptions = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .map_err(|err| {
                tracing::error!("Failed to parse database URL: {}", err);
                panic!("Failed to parse database URL: {}", err);
            })
            .unwrap();

        let mut pool_options = sqlx::pool::PoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .max_connections(100)
            .min_connections(50);

        if cfg!(test) {
            pool_options = pool_options.max_connections(1).test_before_acquire(true);
        }

        pool_options.connect_lazy_with(opts)
    })
}

#[cfg(test)]
pub async fn tear_down() {
    // Delete all tables
    for table in ["account_user", "project", "user_role"] {
        sqlx::query(&format!("TRUNCATE TABLE {} CASCADE", table))
            .execute(connection_pool())
            .await
            .unwrap();
    }
}
