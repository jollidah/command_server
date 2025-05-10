use sqlx::PgConnection;

use crate::{domain::auth::UserAccountAggregate, errors::ServiceError};

pub async fn insert_user_account(
    input: &UserAccountAggregate,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        "INSERT INTO account_user (
            id,
            email,
            name,
            phone_num,
            password,
            verified,
            create_dt
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
         ",
        input.id,
        input.email,
        input.name,
        input.phone_num,
        input.password,
        input.verified,
        input.create_dt
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn get_user_account_by_email(
    email: &String,
    conn: &'static sqlx::PgPool,
) -> Result<UserAccountAggregate, ServiceError> {
    sqlx::query_as!(
        UserAccountAggregate,
        "SELECT * FROM account_user WHERE email = $1",
        email
    )
    .fetch_one(conn)
    .await
    .map_err(Into::<ServiceError>::into)
}

pub async fn update_user_account(
    input: &UserAccountAggregate,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        "UPDATE account_user
            SET
                email = $1,
                phone_num = $2,
                password = $3,
                verified = $4
        WHERE id = $5",
        input.email,
        input.phone_num,
        input.password,
        input.verified,
        input.id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adapter::repositories::{connection_pool, interfaces::TExecutor, tear_down, SqlExecutor},
        domain::auth::UserAccountAggregate,
    };
    use chrono::Utc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_insert_user_account() {
        // GIVEN
        tear_down().await;

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        let user_id = Uuid::new_v4();
        let user_account = UserAccountAggregate {
            id: user_id,
            email: format!("{}@test.com", Uuid::new_v4()),
            name: "Test User".to_string(),
            phone_num: "1234567890".to_string(),
            password: format!("password{}", Uuid::new_v4()),
            verified: false,
            create_dt: Utc::now(),
        };

        // WHEN
        insert_user_account(&user_account, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        let inserted_user = sqlx::query_as!(
            UserAccountAggregate,
            "SELECT *
            FROM account_user
            WHERE id = $1",
            user_id
        )
        .fetch_one(connection_pool())
        .await
        .unwrap();

        // THEN
        assert_eq!(inserted_user.id, user_id);
        assert_eq!(inserted_user.email, user_account.email);
        assert_eq!(inserted_user.name, user_account.name);
        assert_eq!(inserted_user.phone_num, user_account.phone_num);
        assert_eq!(inserted_user.password, user_account.password);
    }

    #[tokio::test]
    async fn test_get_user_account_by_email() {
        // GIVEN
        tear_down().await;

        let user_id = Uuid::new_v4();
        let user_account = UserAccountAggregate {
            id: user_id,
            email: format!("{}@test.com", Uuid::new_v4()),
            name: "Test User".to_string(),
            phone_num: "1234567890".to_string(),
            password: "password123".to_string(),
            verified: false,
            create_dt: Utc::now(),
        };

        sqlx::query!(
            "INSERT INTO account_user (
                id,
                email,
                name,
                phone_num,
                password,
                verified,
                create_dt
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            user_account.id,
            user_account.email,
            user_account.name,
            user_account.phone_num,
            user_account.password,
            user_account.verified,
            user_account.create_dt
        )
        .execute(connection_pool())
        .await
        .unwrap();

        // WHEN
        let fetched_user = get_user_account_by_email(&user_account.email, connection_pool())
            .await
            .unwrap();

        // THEN
        assert_eq!(fetched_user.id, user_id);
        assert_eq!(fetched_user.email, user_account.email);
        assert_eq!(fetched_user.name, user_account.name);
        assert_eq!(fetched_user.phone_num, user_account.phone_num);
        assert_eq!(fetched_user.password, user_account.password);
    }

    #[tokio::test]
    async fn test_update_user_account() {
        // GIVEN
        tear_down().await;

        let user_id = Uuid::new_v4();
        let mut user_account = UserAccountAggregate {
            id: user_id,
            email: format!("{}@test.com", Uuid::new_v4()),
            name: "Test User".to_string(),
            phone_num: "1234567890".to_string(),
            password: "password123".to_string(),
            verified: false,
            create_dt: Utc::now(),
        };
        sqlx::query!(
            "INSERT INTO account_user (
                id,
                email,
                name,
                phone_num,
                password,
                verified,
                create_dt
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            user_account.id,
            user_account.email,
            user_account.name,
            user_account.phone_num,
            user_account.password,
            user_account.verified,
            user_account.create_dt
        )
        .execute(connection_pool())
        .await
        .unwrap();

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // WHEN
        user_account.verified = true;
        user_account.phone_num = "0987654321".to_string();
        update_user_account(&user_account, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let updated_user = sqlx::query_as!(
            UserAccountAggregate,
            "SELECT * FROM account_user WHERE id = $1",
            user_id
        )
        .fetch_one(connection_pool())
        .await
        .unwrap();

        assert_eq!(updated_user.phone_num, "0987654321");
        assert_eq!(updated_user.verified, true);
    }
}
