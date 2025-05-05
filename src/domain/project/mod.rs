use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::errors::ServiceError;

pub mod commands;

#[allow(unused)]
pub struct ProjectAggregate {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) create_dt: DateTime<Utc>,
    pub(crate) update_dt: DateTime<Utc>,
    pub(crate) version: i64,
}

impl ProjectAggregate {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            create_dt: Utc::now(),
            update_dt: Utc::now(),
            version: 1,
        }
    }
}

pub struct UserRoleEntity {
    pub(crate) project_id: Uuid,
    pub(crate) user_email: String,
    pub(crate) role: UserRole,
    pub(crate) update_dt: DateTime<Utc>,
}

impl UserRoleEntity {
    pub fn new(project_id: Uuid, user_email: String, role: UserRole) -> Self {
        Self {
            project_id,
            user_email,
            role,
            update_dt: Utc::now(),
        }
    }
    pub fn verify_admin(&self) -> Result<(), ServiceError> {
        if self.role != UserRole::Admin {
            return Err(ServiceError::Unauthorized);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Editor,
    Viewer,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::Editor => write!(f, "editor"),
            UserRole::Viewer => write!(f, "viewer"),
        }
    }
}

pub struct VultApiKeyEntity {
    pub(crate) project_id: Uuid,
    pub(crate) api_key: String,
    pub(crate) update_dt: DateTime<Utc>,
}

impl VultApiKeyEntity {
    pub fn new(project_id: Uuid, api_key: String) -> Self {
        Self {
            project_id,
            api_key,
            update_dt: Utc::now(),
        }
    }
}
