use chrono::Utc;
use uuid::Uuid;

use crate::domain::auth::{commands::CreateUserAccount, UserAccountAggregate};

impl From<CreateUserAccount> for UserAccountAggregate {
    fn from(command: CreateUserAccount) -> Self {
        UserAccountAggregate {
            id: Uuid::new_v4(),
            email: command.email,
            name: command.name,
            phone_num: command.phone_num,
            password: command.password,
            verified: false,
            create_dt: Utc::now(),
        }
    }
}
