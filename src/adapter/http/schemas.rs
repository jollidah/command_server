use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct UserAccount {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) phone_num: String,
    pub(crate) verified: bool,
    pub(crate) create_dt: DateTime<Utc>,
}
