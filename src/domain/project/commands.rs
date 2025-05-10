use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::UserRole;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct CreateProject {
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct DeleteProject {
    pub(crate) project_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct ExpelMember {
    pub(crate) project_id: Uuid,
    pub(crate) expelled_email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub(crate) struct AssignRole {
    pub(crate) project_id: Uuid,
    pub(crate) invitee_email: String,
    pub(crate) role: UserRole,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct RegisterVultApiKey {
    pub(crate) project_id: Uuid,
    pub(crate) api_key: Vec<u8>,
}
