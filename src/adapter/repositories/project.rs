use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    domain::project::{ProjectAggregate, UserRole, UserRoleEntity, VultApiKeyEntity},
    errors::ServiceError,
};

pub async fn insert_project(
    input: &ProjectAggregate,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        "INSERT INTO project (
            id,
            name,
            description,
            create_dt,
            update_dt,
            version
        ) VALUES ($1, $2, $3, $4, $5, $6)",
        input.id,
        input.name,
        input.description,
        input.create_dt,
        input.update_dt,
        1
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn get_project(
    id: Uuid,
    conn: &'static sqlx::PgPool,
) -> Result<ProjectAggregate, ServiceError> {
    sqlx::query_as!(ProjectAggregate, "SELECT * FROM project WHERE id = $1", id)
        .fetch_one(conn)
        .await
        .map_err(Into::into)
}

pub async fn delete_project(id: Uuid, trx: &mut PgConnection) -> Result<(), ServiceError> {
    sqlx::query!("DELETE FROM project WHERE id = $1", id)
        .execute(&mut *trx)
        .await
        .map_err(Into::<ServiceError>::into)?;
    sqlx::query!("DELETE FROM user_role WHERE project_id = $1", id)
        .execute(trx)
        .await
        .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn upsert_user_role(
    input: &UserRoleEntity,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        INSERT INTO user_role (
            project_id,
            user_email,
            role,
            update_dt
            )
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (project_id, user_email)
            DO UPDATE SET role = $3, update_dt = $4
        "#,
        input.project_id,
        input.user_email,
        &input.role as &UserRole,
        input.update_dt
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn delete_user_role(
    project_id: Uuid,
    user_email: &String,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        DELETE FROM user_role WHERE project_id = $1 AND user_email = $2
        "#,
        project_id,
        user_email
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn get_user_role(
    project_id: Uuid,
    user_email: &String,
    conn: &'static sqlx::PgPool,
) -> Result<UserRoleEntity, ServiceError> {
    sqlx::query_as!(
        UserRoleEntity,
        r#"
            SELECT
                project_id,
                user_email,
                role AS "role:_",
                update_dt
            FROM user_role WHERE project_id = $1 AND user_email = $2
        "#,
        project_id,
        user_email
    )
    .fetch_one(conn)
    .await
    .map_err(Into::<ServiceError>::into)
}

pub async fn upsert_vult_api_key(
    input: &VultApiKeyEntity,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        INSERT INTO vult_api_key (project_id, api_key, update_dt) VALUES ($1, $2, $3)
        ON CONFLICT (project_id)
        DO UPDATE SET api_key = $2, update_dt = $3
        "#,
        input.project_id,
        input.api_key,
        input.update_dt
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}
#[allow(unused)]
pub async fn get_vult_api_key(
    project_id: Uuid,
    conn: &'static sqlx::PgPool,
) -> Result<VultApiKeyEntity, ServiceError> {
    sqlx::query_as!(
        VultApiKeyEntity,
        r#"
        SELECT
            project_id,
            api_key,
            update_dt
        FROM vult_api_key WHERE project_id = $1
        "#,
        project_id
    )
    .fetch_one(conn)
    .await
    .map_err(Into::<ServiceError>::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adapter::repositories::{
            auth::insert_user_account, connection_pool, interfaces::TExecutor, tear_down,
            SqlExecutor,
        },
        domain::{
            auth::UserAccountAggregate,
            project::{ProjectAggregate, UserRole, UserRoleEntity},
        },
    };
    use chrono::Utc;
    use uuid::Uuid;

    async fn create_project_helper() -> (UserAccountAggregate, ProjectAggregate, UserRoleEntity) {
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();
        let user_account = UserAccountAggregate {
            id: Uuid::new_v4(),
            email: format!("test{}@test.com", Uuid::new_v4()),
            name: "Test User".to_string(),
            phone_num: "1234567890".to_string(),
            password: format!("password{}", Uuid::new_v4()),
            verified: true,
            create_dt: Utc::now(),
        };
        let project = ProjectAggregate {
            id: Uuid::new_v4(),
            name: "Test Project".to_string(),
            description: "Test Description".to_string(),
            create_dt: Utc::now(),
            update_dt: Utc::now(),
            version: 1,
        };
        let user_role = UserRoleEntity {
            project_id: project.id,
            user_email: user_account.email.clone(),
            role: UserRole::Admin,
            update_dt: Utc::now(),
        };
        insert_user_account(&user_account, ext.write().await.transaction())
            .await
            .unwrap();
        insert_project(&project, ext.write().await.transaction())
            .await
            .unwrap();
        upsert_user_role(&user_role, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;
        (user_account, project, user_role)
    }

    #[tokio::test]
    async fn test_insert_project() {
        let project = ProjectAggregate {
            id: Uuid::new_v4(),
            name: "Test Project".to_string(),
            description: "Test Description".to_string(),
            create_dt: Utc::now(),
            update_dt: Utc::now(),
            version: 1,
        };
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();
        insert_project(&project, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        let fetched_project = sqlx::query_as!(
            ProjectAggregate,
            "SELECT * FROM project WHERE id = $1",
            project.id
        )
        .fetch_one(connection_pool())
        .await
        .unwrap();

        assert_eq!(fetched_project.id, project.id);
        assert_eq!(fetched_project.name, project.name);
        assert_eq!(fetched_project.description, project.description);
        assert_eq!(
            fetched_project.create_dt.timestamp_millis(),
            project.create_dt.timestamp_millis()
        );
        assert_eq!(
            fetched_project.update_dt.timestamp_millis(),
            project.update_dt.timestamp_millis()
        );
        assert_eq!(fetched_project.version, project.version);
    }

    #[tokio::test]
    async fn test_delete_project() {
        // GIVEN
        tear_down().await;
        let (_, project, user_role) = create_project_helper().await;
        let vult_api_key = VultApiKeyEntity {
            project_id: project.id,
            api_key: "test_vultr_api_key".to_string(),
            update_dt: Utc::now(),
        };
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();
        upsert_vult_api_key(&vult_api_key, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();

        // Verify data insertion
        let fetched_project = get_project(project.id, connection_pool()).await.unwrap();
        assert_eq!(fetched_project.id, project.id);

        let fetched_user_role = get_user_role(project.id, &user_role.user_email, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_user_role.project_id, user_role.project_id);

        let fetched_vult_api_key = get_vult_api_key(project.id, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_vult_api_key.project_id, vult_api_key.project_id);

        // WHEN
        ext.write().await.begin().await.unwrap();
        delete_project(project.id, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        assert!(matches!(
            get_project(project.id, connection_pool()).await,
            Err(ServiceError::RowNotFound)
        ));
        assert!(matches!(
            get_user_role(project.id, &user_role.user_email, connection_pool()).await,
            Err(ServiceError::RowNotFound)
        ));
        assert!(matches!(
            get_vult_api_key(project.id, connection_pool()).await,
            Err(ServiceError::RowNotFound)
        ));
    }

    #[tokio::test]
    async fn test_get_project() {
        let project = ProjectAggregate {
            id: Uuid::new_v4(),
            name: "Test Project".to_string(),
            description: "Test Description".to_string(),
            create_dt: Utc::now(),
            update_dt: Utc::now(),
            version: 1,
        };
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();
        insert_project(&project, ext.write().await.transaction())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_upsert_user_role() {
        // GIVEN
        tear_down().await;
        let user_account = UserAccountAggregate {
            id: Uuid::new_v4(),
            email: format!("test{}@test.com", Uuid::new_v4()),
            name: "Test User".to_string(),
            phone_num: "1234567890".to_string(),
            password: format!("password{}", Uuid::new_v4()),
            verified: true,
            create_dt: Utc::now(),
        };
        let project = ProjectAggregate {
            id: Uuid::new_v4(),
            name: "Test Project".to_string(),
            description: "Test Description".to_string(),
            create_dt: Utc::now(),
            update_dt: Utc::now(),
            version: 1,
        };
        let mut user_role = UserRoleEntity {
            project_id: project.id,
            user_email: user_account.email.clone(),
            role: UserRole::Admin,
            update_dt: Utc::now(),
        };

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();
        insert_user_account(&user_account, ext.write().await.transaction())
            .await
            .unwrap();
        insert_project(&project, ext.write().await.transaction())
            .await
            .unwrap();

        // WHEN (Insert User Role)
        upsert_user_role(&user_role, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        let fetched_user_role = sqlx::query_as!(
            UserRoleEntity,
            r#"
            SELECT
                project_id,
                user_email,
                role AS "role:_",
                update_dt
            FROM user_role WHERE project_id = $1 AND user_email = $2
            "#,
            user_role.project_id,
            user_role.user_email
        )
        .fetch_one(connection_pool())
        .await
        .unwrap();

        assert_eq!(fetched_user_role.project_id, user_role.project_id);
        assert_eq!(fetched_user_role.user_email, user_role.user_email);
        assert_eq!(fetched_user_role.role, user_role.role);
        assert_eq!(
            fetched_user_role.update_dt.timestamp_millis(),
            user_role.update_dt.timestamp_millis()
        );

        // Change Data
        user_role.role = UserRole::Editor;
        ext.write().await.begin().await.unwrap();
        upsert_user_role(&user_role, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_user_role = sqlx::query_as!(
            UserRoleEntity,
            r#"
            SELECT
                project_id,
                user_email,
                role AS "role:_",
                update_dt
            FROM user_role WHERE project_id = $1 AND user_email = $2
            "#,
            user_role.project_id,
            user_role.user_email
        )
        .fetch_one(connection_pool())
        .await
        .unwrap();

        assert_eq!(fetched_user_role.project_id, user_role.project_id);
        assert_eq!(fetched_user_role.user_email, user_role.user_email);
        assert_eq!(fetched_user_role.role, user_role.role);
        assert_eq!(
            fetched_user_role.update_dt.timestamp_millis(),
            user_role.update_dt.timestamp_millis()
        );
    }

    #[tokio::test]
    async fn test_delete_user_role() {
        // GIVEN
        tear_down().await;
        let (user_account, project, _) = create_project_helper().await;

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // WHEN
        delete_user_role(
            project.id,
            &user_account.email,
            ext.write().await.transaction(),
        )
        .await
        .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        matches!(
            get_user_role(project.id, &user_account.email, connection_pool()).await,
            Err(ServiceError::RowNotFound)
        );
    }

    #[tokio::test]
    async fn test_upsert_vult_api_key() {
        // GIVEN
        tear_down().await;
        let (_, project, _) = create_project_helper().await;
        let mut vult_api_key = VultApiKeyEntity {
            project_id: project.id,
            api_key: "test_vultr_api_key".to_string(),
            update_dt: Utc::now(),
        };
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // WHEN
        upsert_vult_api_key(&vult_api_key, ext.write().await.transaction())
            .await
            .unwrap();

        ext.write().await.commit().await.unwrap();

        let fetched_vult_api_key = get_vult_api_key(project.id, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_vult_api_key.project_id, vult_api_key.project_id);
        assert_eq!(fetched_vult_api_key.api_key, vult_api_key.api_key);

        ext.write().await.begin().await.unwrap();
        // Change Data
        vult_api_key.api_key = "test_vultr_api_key_2".to_string();
        upsert_vult_api_key(&vult_api_key, ext.write().await.transaction())
            .await
            .unwrap();

        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_vult_api_key = get_vult_api_key(project.id, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_vult_api_key.project_id, vult_api_key.project_id);
        assert_eq!(fetched_vult_api_key.api_key, vult_api_key.api_key);
    }

    #[tokio::test]
    async fn test_get_vult_api_key() {
        // GIVEN
        tear_down().await;
        let (_, project, _) = create_project_helper().await;
        let vult_api_key = VultApiKeyEntity {
            project_id: project.id,
            api_key: "test_vultr_api_key".to_string(),
            update_dt: Utc::now(),
        };
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();
        upsert_vult_api_key(&vult_api_key, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_vult_api_key = get_vult_api_key(project.id, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_vult_api_key.project_id, vult_api_key.project_id);
        assert_eq!(fetched_vult_api_key.api_key, vult_api_key.api_key);
    }
}
