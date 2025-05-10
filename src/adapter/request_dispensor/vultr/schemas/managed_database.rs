use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapter::request_dispensor::vultr::{interfaces::ExecuteVultrCommand, VultrClient},
    errors::ServiceError,
};

use super::extract_schema_from_response;

#[derive(Serialize, Deserialize)]
pub struct ManagedDatabase {
    id: Uuid,
    date_created: DateTime<Utc>,
    plan: String,
    plan_disk: i64,
    plan_ram: i64,
    plan_vcpus: i64,
    plan_replicas: i64,
    region: String,
    database_engine: String,
    database_engine_version: i64,
    vpc_id: Uuid,
    status: String,
    label: String,
    tag: String,
    dbname: String,
    host: String,
    public_host: String,
    user: String,
    password: String,
    port: i64,
    maintenance_dow: String,
    maintenance_time: String,
    latest_backup: String,
    trusted_ips: Vec<String>,
    mysql_sql_modes: Vec<String>,
    mysql_require_primary_key: bool,
    mysql_slow_query_log: bool,
    cluster_time_zone: String,
    read_replicas: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DatabasePlan {
    id: String,
    number_of_nodes: i64,
    #[serde(rename = "type")]
    database_type: String,
    vcpu_count: i64,
    ram: i64,
    disk: i64,
    monthly_cost: i64,
    supported_engines: HashMap<DatabaseEngine, bool>,
    max_connections: HashMap<DatabaseEngine, i64>,
    locations: Vec<String>,
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseEngine {
    Mysql,
    Pg,
    Valkey,
    Kafka,
}

pub struct ManagedDatabaseCommandFactory;
impl ManagedDatabaseCommandFactory {
    pub fn list_managed_database_plans() -> ListManagedDatabasePlans {
        ListManagedDatabasePlans {}
    }
    pub fn list_managed_database() -> ListManagedDatabase {
        ListManagedDatabase {}
    }
    pub fn create_managed_database(
        database_engine: DatabaseEngine,
        database_engine_version: i64,
        region: String,
        plan: String,
        label: String,
    ) -> CreateManagedDatabase {
        CreateManagedDatabase {
            database_engine,
            database_engine_version,
            region,
            plan,
            label,
        }
    }
    pub fn get_managed_database(database_id: Uuid) -> GetManagedDatabase {
        GetManagedDatabase { database_id }
    }
    pub fn update_managed_database(
        database_id: Uuid,
        plan: String,
        label: String,
    ) -> UpdateManagedDatabase {
        UpdateManagedDatabase {
            database_id,
            plan,
            label,
        }
    }
    pub fn delete_managed_database(database_id: Uuid) -> DeleteManagedDatabase {
        DeleteManagedDatabase { database_id }
    }
}
#[derive(Serialize)]
pub struct ListManagedDatabasePlans;
#[derive(Serialize)]
pub struct ListManagedDatabase;
#[derive(Serialize)]
pub struct CreateManagedDatabase {
    database_engine: DatabaseEngine,
    database_engine_version: i64,
    region: String,
    plan: String,
    label: String,
}
#[derive(Serialize)]
pub struct GetManagedDatabase {
    database_id: Uuid,
}
#[derive(Serialize)]
pub struct UpdateManagedDatabase {
    #[serde(skip_serializing)]
    database_id: Uuid,
    plan: String,
    label: String,
}
#[derive(Serialize)]
pub struct DeleteManagedDatabase {
    database_id: Uuid,
}
#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for ListManagedDatabasePlans {
    async fn execute(
        self,
        vultr_client: &VultrClient,
    ) -> Result<Vec<ManagedDatabase>, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, "databases/plans".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Vec<ManagedDatabase>>(response, "plans").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for ListManagedDatabase {
    async fn execute(
        self,
        vultr_client: &VultrClient,
    ) -> Result<Vec<ManagedDatabase>, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, "databases".to_string())
            .send()
            .await?;
        extract_schema_from_response::<Vec<ManagedDatabase>>(response, "databases").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for CreateManagedDatabase {
    async fn execute(self, vultr_client: &VultrClient) -> Result<ManagedDatabase, ServiceError> {
        let response = vultr_client
            .build_request(Method::POST, "databases".to_string())
            .send()
            .await?;
        extract_schema_from_response::<ManagedDatabase>(response, "database").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for GetManagedDatabase {
    async fn execute(self, vultr_client: &VultrClient) -> Result<ManagedDatabase, ServiceError> {
        let response = vultr_client
            .build_request(Method::GET, format!("databases/{}", self.database_id))
            .send()
            .await?;
        extract_schema_from_response::<ManagedDatabase>(response, "database").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for UpdateManagedDatabase {
    async fn execute(self, vultr_client: &VultrClient) -> Result<ManagedDatabase, ServiceError> {
        let response = vultr_client
            .build_request(Method::PUT, format!("databases/{}", self.database_id))
            .send()
            .await?;
        extract_schema_from_response::<ManagedDatabase>(response, "database").await
    }
}

#[allow(refining_impl_trait)]
impl ExecuteVultrCommand for DeleteManagedDatabase {
    async fn execute(self, vultr_client: &VultrClient) -> Result<(), ServiceError> {
        vultr_client
            .build_request(Method::DELETE, format!("databases/{}", self.database_id))
            .send()
            .await?;
        Ok(())
    }
}
