use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct CreateUserAccount {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) phone_num: String,
    // TODO Hash password
    pub(crate) password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct IssueTokens {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct RefreshTokens {
    pub(crate) refresh_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct CreateVerification {
    pub(crate) email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct CheckVerification {
    pub(crate) email: String,
    pub(crate) verification_code: String,
}
