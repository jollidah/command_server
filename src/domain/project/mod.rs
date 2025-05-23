use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{adapter::request_dispensor::vultr::VultrClient, errors::ServiceError};

pub mod commands;
pub mod diagrams;
pub mod enums;

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
    pub fn verify_role(&self, roles: &[UserRole]) -> Result<(), ServiceError> {
        if !roles.contains(&self.role) {
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

pub struct VultrExecutionContext {
    pub vultr_client: &'static VultrClient,
    pub project_id: Uuid,
    pub resource_map: HashMap<String, String>,
}

impl VultrExecutionContext {
    pub fn new(vultr_client: &'static VultrClient, project_id: Uuid) -> Self {
        Self {
            vultr_client,
            project_id,
            resource_map: HashMap::new(),
        }
    }
    fn get_id_with_temp_id(&mut self, temp_id: &String) -> Result<String, ServiceError> {
        self.resource_map
            .get(temp_id)
            .ok_or(ServiceError::NotFound)
            .cloned()
    }
}

// pub struct VultrCommandManager<'a> {
//     pub(crate) command_list: Vec<VultrCommand>,
//     pub(crate) execution_context: VultrExecutionContext,
//     pub(crate) trx: &'a mut PgConnection,
// }

// impl<'a> VultrCommandManager<'a> {
//     pub fn new(
//         command_list: Vec<VultrCommand>,
//         project_id: Uuid,
//         vultr_api_key: String,
//         trx: &'a mut PgConnection,
//     ) -> Self {
//         Self {
//             command_list,
//             execution_context: VultrExecutionContext::new(
//                 get_vultr_client(vultr_api_key.as_str()),
//                 project_id,
//             ),
//             trx,
//         }
//     }

//     pub async fn execute(&mut self) -> Result<(), ServiceError> {
//         for command_wrapper in &self.command_list {
//             let command_name = command_wrapper.get_command_name();
//             let command_data = command_wrapper.get_command_data();
//             let temp_id = command_wrapper.get_temp_id();

//             match command_name {
//                 name if name.contains("Create") => {
//                     let command: VultrCreateCommand = serde_json::from_value(command_data)?;
//                     command
//                         .execute_create_command(&mut self.execution_context, self.trx, temp_id)
//                         .await?;
//                 }
//                 name if name.contains("Update")
//                     || name.contains("Attach")
//                     || name.contains("Detach") =>
//                 {
//                     let command: VultrUpdateCommand = serde_json::from_value(command_data)?;
//                     command
//                         .execute_update_command(&mut self.execution_context, self.trx, temp_id)
//                         .await?;
//                 }
//                 name if name.contains("Delete") => {
//                     let command: VultrDeleteCommand = serde_json::from_value(command_data)?;
//                     command
//                         .execute_delete_command(&mut self.execution_context, self.trx, temp_id)
//                         .await?;
//                 }
//                 _ => return Err(ServiceError::ParseError),
//             }
//         }
//         Ok(())
//     }
// }
