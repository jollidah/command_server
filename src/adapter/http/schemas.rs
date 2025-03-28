use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct SignUp {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) phone_num: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct SignIn {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct Tokens {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct CreateVerification {
    pub(crate) email: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct CheckVerification {
    pub(crate) email: String,
    pub(crate) verification_code: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
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
