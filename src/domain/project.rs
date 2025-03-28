use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;


#[derive(Debug, Deserialize, ToSchema)]
pub(crate) struct AssignRole {
    pub(crate) _project_id: Uuid,
    pub(crate) _inviter_email: String,
    pub(crate) _invitee_email: String,
    pub(crate) _role: String,
}



