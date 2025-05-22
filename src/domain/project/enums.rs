use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Type, Clone)]
#[sqlx(type_name = "ip_type", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum IpType {
    V4,
    V6,
}

#[derive(Serialize, Deserialize, Type, Clone)]
#[sqlx(type_name = "protocol", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Icmp,
    Tcp,
    Udp,
    Gre,
    Esp,
    Ah,
}

#[derive(Serialize, Deserialize, Default, Clone, Type)]
#[sqlx(type_name = "auto_backups", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum BackupStatus {
    Enabled,
    #[default]
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Type)]
#[sqlx(type_name = "database_engine", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum DatabaseEngine {
    Mysql,
    Pg,
}

#[derive(Debug, Serialize, Deserialize, Type, ToSchema, Clone)]
#[sqlx(type_name = "resource_type", rename_all = "snake_case")]
pub enum ResourceType {
    BlockStorage,
    Compute,
    ManagedDatabase,
    ObjectStorage,
    FirewallGroup,
    FirewallRule,
}
