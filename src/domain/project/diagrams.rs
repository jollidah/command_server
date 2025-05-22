use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::enums::{BackupStatus, DatabaseEngine, IpType, Protocol};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockStorage {
    #[serde(skip)]
    pub project_id: Uuid,
    #[serde(skip)]
    pub y: i64,
    #[serde(skip)]
    pub x: i64,
    // Vultr won't return region_id
    pub region_id: Option<String>, // e.g."ewr"
    pub id: Uuid,
    pub mount_id: String,
    pub attached_to_instance: Uuid,
    pub size_gb: i64,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirewallGroup {
    #[serde(skip)]
    pub project_id: Uuid,
    #[serde(skip)]
    pub y: i64,
    #[serde(skip)]
    pub x: i64,
    pub id: Uuid,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FirewallRule {
    #[serde(skip)]
    pub project_id: Uuid,
    #[serde(skip)]
    pub y: i64,
    #[serde(skip)]
    pub x: i64,
    pub id: i64,
    pub action: String,
    pub port: String,
    pub ip_type: IpType,
    pub protocol: Protocol,
    pub subnet: String,
    pub subnet_size: i64,
    pub notes: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Compute {
    #[serde(skip)]
    pub project_id: Uuid,
    #[serde(skip)]
    pub y: i64,
    #[serde(skip)]
    pub x: i64,
    // Vultr won't return region_id
    pub region_id: Option<String>,
    // Vultr won't return auto_backups
    pub auto_backups: Option<BackupStatus>,
    pub id: Uuid,
    pub plan: String,
    pub status: String,
    pub main_ip: String,
    pub label: String,
    pub os_id: i64,
    pub firewall_group_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ManagedDatabase {
    #[serde(skip)]
    pub project_id: Uuid,
    #[serde(skip)]
    pub y: i64,
    #[serde(skip)]
    pub x: i64,
    // Vultr won't return region_id
    pub region_id: Option<String>, // e.g. "ewr"
    pub id: Uuid,
    pub status: String,
    pub plan: String,
    pub database_engine: DatabaseEngine,
    pub database_engine_version: i64,
    pub latest_backup: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ObjectStorage {
    #[serde(skip)]
    pub project_id: Uuid,
    #[serde(skip)]
    pub y: i64,
    #[serde(skip)]
    pub x: i64,
    // Vultr won't return tier_id
    pub tier_id: Option<i64>,
    pub id: Uuid,
    pub cluster_id: i64,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ObjectPosition {
    pub x: i64,
    pub y: i64,
}

pub fn get_diagram_key(project_id: Uuid) -> String {
    format!("project_diagram_{}", project_id)
}

pub fn get_diagram_update_dt(project_id: Uuid) -> String {
    format!("project_diagram_update_dt_{}", project_id)
}
