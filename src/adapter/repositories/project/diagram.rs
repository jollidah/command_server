use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    domain::project::diagrams::{
        BlockStorage, Compute, FirewallGroup, FirewallRule, ManagedDatabase, ObjectStorage,
    },
    errors::ServiceError,
};

pub async fn insert_block_storage(
    input: &BlockStorage,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        INSERT INTO block_storage (
            project_id,
            region,
            id,
            mount_id,
            attached_to_instance,
            size_gb,
            label,
            x,
            y
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        input.project_id,
        input.region,
        input.id,
        input.mount_id,
        input.attached_to_instance,
        input.size_gb,
        input.label,
        input.x,
        input.y
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn update_block_storage(
    input: &BlockStorage,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        UPDATE block_storage
        SET 
            region = $1,
            mount_id = $2,
            attached_to_instance = $3,
            size_gb = $4,
            label = $5,
            x = $6,
            y = $7
        WHERE project_id = $8 AND id = $9
        "#,
        input.region,
        input.mount_id,
        input.attached_to_instance,
        input.size_gb,
        input.label,
        input.x,
        input.y,
        input.project_id,
        input.id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn delete_block_storage(
    project_id: &Uuid,
    id: &Uuid,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        "DELETE FROM block_storage WHERE project_id = $1 AND id = $2",
        project_id,
        id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn list_block_storage(
    project_id: &Uuid,
    conn: &'static sqlx::PgPool,
) -> Result<Vec<BlockStorage>, ServiceError> {
    sqlx::query_as!(
        BlockStorage,
        r#"
        SELECT
            project_id,
            region,
            id,
            mount_id,
            attached_to_instance,
            size_gb,
            label,
            x,
            y
        FROM block_storage 
        WHERE project_id = $1
        "#,
        project_id
    )
    .fetch_all(conn)
    .await
    .map_err(Into::into)
}

pub async fn insert_firewall_group(
    input: &FirewallGroup,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        INSERT INTO firewall_group (
            project_id,
            id,
            description,
            x,
            y
        ) VALUES ($1, $2, $3, $4, $5)
        "#,
        input.project_id,
        input.id,
        input.description,
        input.x,
        input.y
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn update_firewall_group(
    input: &FirewallGroup,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        UPDATE firewall_group
        SET description = $1,
            x = $2,
            y = $3
        WHERE project_id = $4 AND id = $5
        "#,
        input.description,
        input.x,
        input.y,
        input.project_id,
        input.id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn delete_firewall_group(
    project_id: &Uuid,
    id: &Uuid,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        "DELETE FROM firewall_group WHERE project_id = $1 AND id = $2",
        project_id,
        id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn list_firewall_group(
    project_id: &Uuid,
    conn: &'static sqlx::PgPool,
) -> Result<Vec<FirewallGroup>, ServiceError> {
    sqlx::query_as!(
        FirewallGroup,
        r#"
        SELECT
            project_id,
            id,
            description,
            x,
            y
        FROM firewall_group 
        WHERE project_id = $1
        "#,
        project_id
    )
    .fetch_all(conn)
    .await
    .map_err(Into::into)
}

pub async fn insert_firewall_rule(
    input: &FirewallRule,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        INSERT INTO firewall_rule (
            project_id,
            id,
            action,
            port,
            ip_type,
            protocol,
            subnet,
            subnet_size,
            notes,
            x,
            y
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        input.project_id,
        input.id,
        input.action,
        input.port,
        &input.ip_type as _,
        &input.protocol as _,
        input.subnet,
        input.subnet_size,
        input.notes,
        input.x,
        input.y
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn delete_firewall_rule(
    project_id: &Uuid,
    id: &i64,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        "DELETE FROM firewall_rule WHERE project_id = $1 AND id = $2",
        project_id,
        id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn list_firewall_rule(
    project_id: &Uuid,
    conn: &'static sqlx::PgPool,
) -> Result<Vec<FirewallRule>, ServiceError> {
    sqlx::query_as!(
        FirewallRule,
        r#"
        SELECT
            project_id,
            id,
            action,
            port,
            ip_type as "ip_type:_",
            protocol as "protocol:_",
            subnet,
            subnet_size,
            notes,
            x,
            y
        FROM firewall_rule 
        WHERE project_id = $1
        "#,
        project_id
    )
    .fetch_all(conn)
    .await
    .map_err(Into::into)
}

pub async fn insert_compute(input: &Compute, trx: &mut PgConnection) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        INSERT INTO compute (
            project_id,
            region,
            id,
            plan,
            status,
            main_ip,
            label,
            os_id,
            firewall_group_id,
            auto_backups,
            x,
            y
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        input.project_id,
        input.region,
        input.id,
        input.plan,
        input.status,
        input.main_ip,
        input.label,
        input.os_id,
        input.firewall_group_id,
        &input.auto_backups as _,
        input.x,
        input.y
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn update_compute(input: &Compute, trx: &mut PgConnection) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        UPDATE compute
        SET 
            plan = $1,
            status = $2,
            main_ip = $3,
            label = $4,
            os_id = $5,
            firewall_group_id = $6,
            auto_backups = $7,
            x = $8,
            y = $9
        WHERE project_id = $10 AND id = $11
        "#,
        input.plan,
        input.status,
        input.main_ip,
        input.label,
        input.os_id,
        input.firewall_group_id,
        &input.auto_backups as _,
        input.x,
        input.y,
        input.project_id,
        input.id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn delete_compute(
    project_id: &Uuid,
    id: &Uuid,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        "DELETE FROM compute WHERE project_id = $1 AND id = $2",
        project_id,
        id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn list_compute(
    project_id: &Uuid,
    conn: &'static sqlx::PgPool,
) -> Result<Vec<Compute>, ServiceError> {
    sqlx::query_as!(
        Compute,
        r#"
        SELECT
            project_id,
            region,
            id,
            plan,
            status,
            main_ip,
            label,
            os_id,
            firewall_group_id,
            auto_backups as "auto_backups:_",
            x,
            y
        FROM compute 
        WHERE project_id = $1
        "#,
        project_id
    )
    .fetch_all(conn)
    .await
    .map_err(Into::into)
}

pub async fn insert_managed_database(
    input: &ManagedDatabase,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        INSERT INTO managed_database (
            project_id,
            region,
            id,
            status,
            plan,
            database_engine,
            database_engine_version,
            latest_backup,
            label,
            x,
            y
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        input.project_id,
        input.region,
        input.id,
        input.status,
        input.plan,
        &input.database_engine as _,
        input.database_engine_version,
        input.latest_backup,
        input.label,
        input.x,
        input.y
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn update_managed_database(
    input: &ManagedDatabase,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        UPDATE managed_database
        SET 
            status = $1,
            plan = $2,
            database_engine = $3,
            database_engine_version = $4,
            latest_backup = $5,
            label = $6,
            x = $7,
            y = $8
        WHERE project_id = $9 AND id = $10
        "#,
        input.status,
        input.plan,
        &input.database_engine as _,
        input.database_engine_version,
        input.latest_backup,
        input.label,
        input.x,
        input.y,
        input.project_id,
        input.id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn delete_managed_database(
    project_id: &Uuid,
    id: &Uuid,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        "DELETE FROM managed_database WHERE project_id = $1 AND id = $2",
        project_id,
        id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn list_managed_database(
    project_id: &Uuid,
    conn: &'static sqlx::PgPool,
) -> Result<Vec<ManagedDatabase>, ServiceError> {
    sqlx::query_as!(
        ManagedDatabase,
        r#"
        SELECT
            project_id,
            region,
            id,
            status,
            plan,
            database_engine as "database_engine:_",
            database_engine_version,
            latest_backup,
            label,
            x,
            y
        FROM managed_database 
        WHERE project_id = $1
        "#,
        project_id
    )
    .fetch_all(conn)
    .await
    .map_err(Into::into)
}

pub async fn insert_object_storage(
    input: &ObjectStorage,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        INSERT INTO object_storage (
            project_id,
            tier_id,
            id,
            cluster_id,
            label,
            x,
            y
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        input.project_id,
        input.tier_id,
        input.id,
        input.cluster_id,
        input.label,
        input.x,
        input.y
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn update_object_storage(
    input: &ObjectStorage,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        UPDATE object_storage
        SET 
            tier_id = $1,
            cluster_id = $2,
            label = $3,
            x = $4,
            y = $5
        WHERE project_id = $6 AND id = $7
        "#,
        input.tier_id,
        input.cluster_id,
        input.label,
        input.x,
        input.y,
        input.project_id,
        input.id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn delete_object_storage(
    project_id: &Uuid,
    id: &Uuid,
    trx: &mut PgConnection,
) -> Result<(), ServiceError> {
    sqlx::query!(
        "DELETE FROM object_storage WHERE project_id = $1 AND id = $2",
        project_id,
        id
    )
    .execute(trx)
    .await
    .map_err(Into::<ServiceError>::into)?;
    Ok(())
}

pub async fn list_object_storage(
    project_id: &Uuid,
    conn: &'static sqlx::PgPool,
) -> Result<Vec<ObjectStorage>, ServiceError> {
    sqlx::query_as!(
        ObjectStorage,
        r#"
        SELECT
            project_id,
            tier_id,
            id,
            cluster_id,
            label,
            x,
            y
        FROM object_storage
        WHERE project_id = $1
        "#,
        project_id
    )
    .fetch_all(conn)
    .await
    .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adapter::repositories::{
            auth::insert_user_account, connection_pool, interfaces::TExecutor,
            project::workspace::insert_project, tear_down, SqlExecutor,
        },
        domain::{
            auth::UserAccountAggregate,
            project::{
                diagrams::{
                    BlockStorage, Compute, FirewallGroup, FirewallRule, ManagedDatabase,
                    ObjectStorage,
                },
                enums::{BackupStatus, DatabaseEngine, IpType, Protocol},
                ProjectAggregate,
            },
        },
    };
    use chrono::Utc;
    use uuid::Uuid;

    async fn create_project_helper() -> (UserAccountAggregate, ProjectAggregate) {
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
        insert_user_account(&user_account, ext.write().await.transaction())
            .await
            .unwrap();
        insert_project(&project, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;
        (user_account, project)
    }

    async fn get_block_storage(
        input: &BlockStorage,
        conn: &'static sqlx::PgPool,
    ) -> Result<BlockStorage, ServiceError> {
        sqlx::query_as!(
            BlockStorage,
            r#"
            SELECT 
                project_id,
                region,
                id,
                mount_id,
                attached_to_instance,
                size_gb,
                label,
                x,
                y
            FROM block_storage 
            WHERE project_id = $1 AND id = $2
            "#,
            input.project_id,
            input.id
        )
        .fetch_one(conn)
        .await
        .map_err(Into::into)
    }
    async fn get_firewall_group(
        input: &FirewallGroup,
        conn: &'static sqlx::PgPool,
    ) -> Result<FirewallGroup, ServiceError> {
        sqlx::query_as!(
            FirewallGroup,
            r#"
            SELECT 
                project_id,
                id,
                description,
                x,
                y
            FROM firewall_group 
            WHERE project_id = $1 AND id = $2
            "#,
            input.project_id,
            input.id
        )
        .fetch_one(conn)
        .await
        .map_err(Into::into)
    }

    pub async fn get_firewall_rule(
        input: &FirewallRule,
        conn: &'static sqlx::PgPool,
    ) -> Result<FirewallRule, ServiceError> {
        sqlx::query_as!(
            FirewallRule,
            r#"
            SELECT 
                project_id,
                id,
                action,
                port,
                ip_type as "ip_type:_",
                protocol as "protocol:_",
                subnet,
                subnet_size,
                notes,
                x,
                y
            FROM firewall_rule 
            WHERE project_id = $1 AND id = $2
            "#,
            input.project_id,
            input.id
        )
        .fetch_one(conn)
        .await
        .map_err(Into::into)
    }

    pub async fn get_compute(
        input: &Compute,
        conn: &'static sqlx::PgPool,
    ) -> Result<Compute, ServiceError> {
        sqlx::query_as!(
            Compute,
            r#"
            SELECT 
                project_id,
                region,
                id,
                plan,
                status,
                main_ip,
                label,
                os_id,
                firewall_group_id,
                auto_backups as "auto_backups:_",
                x,
                y
            FROM compute 
            WHERE project_id = $1 AND id = $2
            "#,
            input.project_id,
            input.id
        )
        .fetch_one(conn)
        .await
        .map_err(Into::into)
    }

    pub async fn get_managed_database(
        input: &ManagedDatabase,
        conn: &'static sqlx::PgPool,
    ) -> Result<ManagedDatabase, ServiceError> {
        sqlx::query_as!(
            ManagedDatabase,
            r#"
            SELECT 
                project_id,
                region,
                id,
                status,
                plan,
                database_engine as "database_engine:_",
                database_engine_version,
                latest_backup,
                label,
                x,
                y
            FROM managed_database 
            WHERE project_id = $1 AND id = $2
            "#,
            input.project_id,
            input.id
        )
        .fetch_one(conn)
        .await
        .map_err(Into::into)
    }

    pub async fn get_object_storage(
        input: &ObjectStorage,
        conn: &'static sqlx::PgPool,
    ) -> Result<ObjectStorage, ServiceError> {
        sqlx::query_as!(
            ObjectStorage,
            r#"
            SELECT 
                project_id,
                tier_id,
                id,
                cluster_id,
                label,
                x,
                y
            FROM object_storage 
            WHERE project_id = $1 AND id = $2
            "#,
            input.project_id,
            input.id
        )
        .fetch_one(conn)
        .await
        .map_err(Into::into)
    }

    #[tokio::test]
    async fn test_block_storage_crud() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let block_storage = BlockStorage {
            project_id: project.id,
            region: Some("region1".to_string()),
            id: Uuid::new_v4(),
            mount_id: "mount1".to_string(),
            attached_to_instance: Uuid::new_v4(),
            size_gb: 100,
            label: "test-block-storage".to_string(),
            x: 0,
            y: 0,
        };

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // WHEN (Insert)
        insert_block_storage(&block_storage, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();

        // THEN
        let fetched_block_storage = get_block_storage(&block_storage, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_block_storage.project_id, block_storage.project_id);
        assert_eq!(fetched_block_storage.id, block_storage.id);
        assert_eq!(fetched_block_storage.mount_id, block_storage.mount_id);
        assert_eq!(
            fetched_block_storage.attached_to_instance,
            block_storage.attached_to_instance
        );
        assert_eq!(fetched_block_storage.size_gb, block_storage.size_gb);
        assert_eq!(fetched_block_storage.label, block_storage.label);

        // WHEN (Update)
        let mut updated_block_storage = block_storage.clone();
        updated_block_storage.label = "updated-label".to_string();
        ext.write().await.begin().await.unwrap();
        update_block_storage(&updated_block_storage, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();

        // THEN
        let fetched_block_storage = get_block_storage(&updated_block_storage, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_block_storage.label, updated_block_storage.label);

        // WHEN (Delete)
        ext.write().await.begin().await.unwrap();
        delete_block_storage(
            &block_storage.project_id,
            &block_storage.id,
            ext.write().await.transaction(),
        )
        .await
        .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        assert!(matches!(
            get_block_storage(&block_storage, connection_pool()).await,
            Err(ServiceError::NotFound)
        ));
    }

    #[tokio::test]
    async fn test_firewall_group_crud() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let firewall_group = FirewallGroup {
            project_id: project.id,
            id: Uuid::new_v4(),
            description: "test-firewall-group".to_string(),
            x: 0,
            y: 0,
        };

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // WHEN (Insert)
        insert_firewall_group(&firewall_group, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_firewall_group = get_firewall_group(&firewall_group, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_firewall_group.project_id, firewall_group.project_id);
        assert_eq!(fetched_firewall_group.id, firewall_group.id);
        assert_eq!(
            fetched_firewall_group.description,
            firewall_group.description
        );

        // WHEN (Update)
        let mut updated_firewall_group = firewall_group.clone();
        updated_firewall_group.description = "updated-description".to_string();
        ext.write().await.begin().await.unwrap();
        update_firewall_group(&updated_firewall_group, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_firewall_group = get_firewall_group(&updated_firewall_group, connection_pool())
            .await
            .unwrap();
        assert_eq!(
            fetched_firewall_group.description,
            updated_firewall_group.description
        );

        // WHEN (Delete)
        ext.write().await.begin().await.unwrap();
        delete_firewall_group(
            &firewall_group.project_id,
            &firewall_group.id,
            ext.write().await.transaction(),
        )
        .await
        .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        assert!(matches!(
            get_firewall_group(&firewall_group, connection_pool()).await,
            Err(ServiceError::NotFound)
        ));
    }

    #[tokio::test]
    async fn test_firewall_rule_crud() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let firewall_rule = FirewallRule {
            project_id: project.id,
            id: 1,
            action: "allow".to_string(),
            port: "80".to_string(),
            ip_type: IpType::V4,
            protocol: Protocol::Tcp,
            subnet: "0.0.0.0".to_string(),
            subnet_size: 24,
            notes: "test-rule".to_string(),
            x: 0,
            y: 0,
        };

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // WHEN (Insert)
        insert_firewall_rule(&firewall_rule, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_firewall_rule = get_firewall_rule(&firewall_rule, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_firewall_rule.project_id, firewall_rule.project_id);
        assert_eq!(fetched_firewall_rule.id, firewall_rule.id);
        assert_eq!(fetched_firewall_rule.action, firewall_rule.action);
        assert_eq!(fetched_firewall_rule.port, firewall_rule.port);
        assert_eq!(fetched_firewall_rule.subnet, firewall_rule.subnet);
        assert_eq!(fetched_firewall_rule.subnet_size, firewall_rule.subnet_size);
        assert_eq!(fetched_firewall_rule.notes, firewall_rule.notes);

        // WHEN (Delete)
        ext.write().await.begin().await.unwrap();
        delete_firewall_rule(
            &firewall_rule.project_id,
            &firewall_rule.id,
            ext.write().await.transaction(),
        )
        .await
        .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        assert!(matches!(
            get_firewall_rule(&firewall_rule, connection_pool()).await,
            Err(ServiceError::NotFound)
        ));
    }

    #[tokio::test]
    async fn test_compute_crud() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let compute = Compute {
            project_id: project.id,
            region: Some("region1".to_string()),
            id: Uuid::new_v4(),
            plan: "vc2-1c-1gb".to_string(),
            status: "active".to_string(),
            main_ip: "192.168.1.1".to_string(),
            label: "test-compute".to_string(),
            os_id: 1,
            firewall_group_id: "default".to_string(),
            auto_backups: Some(BackupStatus::Disabled),
            x: 0,
            y: 0,
        };

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // WHEN (Insert)
        insert_compute(&compute, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_compute = get_compute(&compute, connection_pool()).await.unwrap();
        assert_eq!(fetched_compute.project_id, compute.project_id);
        assert_eq!(fetched_compute.id, compute.id);
        assert_eq!(fetched_compute.plan, compute.plan);
        assert_eq!(fetched_compute.status, compute.status);
        assert_eq!(fetched_compute.main_ip, compute.main_ip);
        assert_eq!(fetched_compute.label, compute.label);
        assert_eq!(fetched_compute.os_id, compute.os_id);
        assert_eq!(fetched_compute.firewall_group_id, compute.firewall_group_id);

        // WHEN (Update)
        let mut updated_compute = compute.clone();
        updated_compute.label = "updated-compute".to_string();
        ext.write().await.begin().await.unwrap();
        update_compute(&updated_compute, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_compute = get_compute(&updated_compute, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_compute.label, updated_compute.label);

        // WHEN (Delete)
        ext.write().await.begin().await.unwrap();
        delete_compute(
            &compute.project_id,
            &compute.id,
            ext.write().await.transaction(),
        )
        .await
        .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        assert!(matches!(
            get_compute(&compute, connection_pool()).await,
            Err(ServiceError::NotFound)
        ));
    }

    #[tokio::test]
    async fn test_managed_database_crud() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let managed_database = ManagedDatabase {
            project_id: project.id,
            region: Some("region1".to_string()),
            id: Uuid::new_v4(),
            status: "active".to_string(),
            plan: "db-s-1vcpu-1gb".to_string(),
            database_engine: DatabaseEngine::Mysql,
            database_engine_version: 8,
            latest_backup: "2024-03-20".to_string(),
            label: "test-database".to_string(),
            x: 0,
            y: 0,
        };

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // WHEN (Insert)
        insert_managed_database(&managed_database, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_database = get_managed_database(&managed_database, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_database.project_id, managed_database.project_id);
        assert_eq!(fetched_database.id, managed_database.id);
        assert_eq!(fetched_database.status, managed_database.status);
        assert_eq!(fetched_database.plan, managed_database.plan);
        assert_eq!(
            fetched_database.database_engine,
            managed_database.database_engine
        );
        assert_eq!(
            fetched_database.database_engine_version,
            managed_database.database_engine_version
        );
        assert_eq!(
            fetched_database.latest_backup,
            managed_database.latest_backup
        );
        assert_eq!(fetched_database.label, managed_database.label);

        // WHEN (Update)
        let mut updated_database = managed_database.clone();
        updated_database.label = "updated-database".to_string();
        ext.write().await.begin().await.unwrap();
        update_managed_database(&updated_database, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_database = get_managed_database(&updated_database, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_database.label, updated_database.label);

        // WHEN (Delete)
        ext.write().await.begin().await.unwrap();
        delete_managed_database(
            &managed_database.project_id,
            &managed_database.id,
            ext.write().await.transaction(),
        )
        .await
        .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        assert!(matches!(
            get_managed_database(&managed_database, connection_pool()).await,
            Err(ServiceError::NotFound)
        ));
    }

    #[tokio::test]
    async fn test_object_storage_crud() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let object_storage = ObjectStorage {
            project_id: project.id,
            tier_id: Some(1),
            id: Uuid::new_v4(),
            cluster_id: 1,
            label: "test-storage".to_string(),
            x: 0,
            y: 0,
        };

        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // WHEN (Insert)
        insert_object_storage(&object_storage, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_storage = get_object_storage(&object_storage, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_storage.project_id, object_storage.project_id);
        assert_eq!(fetched_storage.id, object_storage.id);
        assert_eq!(fetched_storage.tier_id, object_storage.tier_id);
        assert_eq!(fetched_storage.cluster_id, object_storage.cluster_id);
        assert_eq!(fetched_storage.label, object_storage.label);

        // WHEN (Update)
        let mut updated_storage = object_storage.clone();
        updated_storage.label = "updated-storage".to_string();
        ext.write().await.begin().await.unwrap();
        update_object_storage(&updated_storage, ext.write().await.transaction())
            .await
            .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        let fetched_storage = get_object_storage(&updated_storage, connection_pool())
            .await
            .unwrap();
        assert_eq!(fetched_storage.label, updated_storage.label);

        // WHEN (Delete)
        ext.write().await.begin().await.unwrap();
        delete_object_storage(
            &object_storage.project_id,
            &object_storage.id,
            ext.write().await.transaction(),
        )
        .await
        .unwrap();
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // THEN
        assert!(matches!(
            get_object_storage(&object_storage, connection_pool()).await,
            Err(ServiceError::NotFound)
        ));
    }

    #[tokio::test]
    async fn test_list_compute() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // Create multiple compute instances
        let computes = vec![
            Compute {
                project_id: project.id,
                region: Some("region1".to_string()),
                id: Uuid::new_v4(),
                plan: "vc2-1c-1gb".to_string(),
                status: "active".to_string(),
                main_ip: "192.168.1.1".to_string(),
                label: "test-compute-1".to_string(),
                os_id: 1,
                firewall_group_id: "default".to_string(),
                auto_backups: Some(BackupStatus::Disabled),
                x: 0,
                y: 0,
            },
            Compute {
                project_id: project.id,
                region: Some("region1".to_string()),
                id: Uuid::new_v4(),
                plan: "vc2-2c-2gb".to_string(),
                status: "active".to_string(),
                main_ip: "192.168.1.2".to_string(),
                label: "test-compute-2".to_string(),
                os_id: 1,
                firewall_group_id: "default".to_string(),
                auto_backups: Some(BackupStatus::Disabled),
                x: 0,
                y: 0,
            },
        ];

        for compute in &computes {
            insert_compute(compute, ext.write().await.transaction())
                .await
                .unwrap();
        }
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // WHEN
        let fetched_computes = list_compute(&project.id, connection_pool()).await.unwrap();

        // THEN
        assert_eq!(fetched_computes.len(), 2);
        assert!(fetched_computes.iter().any(|c| c.label == "test-compute-1"));
        assert!(fetched_computes.iter().any(|c| c.label == "test-compute-2"));
    }

    #[tokio::test]
    async fn test_list_managed_database() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // Create multiple managed databases
        let databases = vec![
            ManagedDatabase {
                project_id: project.id,
                region: Some("region1".to_string()),
                id: Uuid::new_v4(),
                status: "active".to_string(),
                plan: "db-s-1vcpu-1gb".to_string(),
                database_engine: DatabaseEngine::Mysql,
                database_engine_version: 8,
                latest_backup: "2024-03-20".to_string(),
                label: "test-db-1".to_string(),
                x: 0,
                y: 0,
            },
            ManagedDatabase {
                project_id: project.id,
                region: Some("region1".to_string()),
                id: Uuid::new_v4(),
                status: "active".to_string(),
                plan: "db-s-2vcpu-2gb".to_string(),
                database_engine: DatabaseEngine::Pg,
                database_engine_version: 14,
                latest_backup: "2024-03-20".to_string(),
                label: "test-db-2".to_string(),
                x: 0,
                y: 0,
            },
        ];

        for database in &databases {
            insert_managed_database(database, ext.write().await.transaction())
                .await
                .unwrap();
        }
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // WHEN
        let fetched_databases = list_managed_database(&project.id, connection_pool())
            .await
            .unwrap();

        // THEN
        assert_eq!(fetched_databases.len(), 2);
        assert!(fetched_databases
            .iter()
            .any(|d| d.label == "test-db-1" && d.database_engine == DatabaseEngine::Mysql));
        assert!(fetched_databases
            .iter()
            .any(|d| d.label == "test-db-2" && d.database_engine == DatabaseEngine::Pg));
    }

    #[tokio::test]
    async fn test_list_block_storage() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // Create multiple block storages
        let block_storages = vec![
            BlockStorage {
                project_id: project.id,
                region: Some("region1".to_string()),
                id: Uuid::new_v4(),
                mount_id: "mount1".to_string(),
                attached_to_instance: Uuid::new_v4(),
                size_gb: 100,
                label: "test-storage-1".to_string(),
                x: 0,
                y: 0,
            },
            BlockStorage {
                project_id: project.id,
                region: Some("region1".to_string()),
                id: Uuid::new_v4(),
                mount_id: "mount2".to_string(),
                attached_to_instance: Uuid::new_v4(),
                size_gb: 200,
                label: "test-storage-2".to_string(),
                x: 0,
                y: 0,
            },
        ];

        for storage in &block_storages {
            insert_block_storage(storage, ext.write().await.transaction())
                .await
                .unwrap();
        }
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // WHEN
        let fetched_storages = list_block_storage(&project.id, connection_pool())
            .await
            .unwrap();

        // THEN
        assert_eq!(fetched_storages.len(), 2);
    }

    #[tokio::test]
    async fn test_list_firewall_group() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // Create multiple firewall groups
        let firewall_groups = vec![
            FirewallGroup {
                project_id: project.id,
                id: Uuid::new_v4(),
                description: "test-group-1".to_string(),
                x: 0,
                y: 0,
            },
            FirewallGroup {
                project_id: project.id,
                id: Uuid::new_v4(),
                description: "test-group-2".to_string(),
                x: 0,
                y: 0,
            },
        ];

        for group in &firewall_groups {
            insert_firewall_group(group, ext.write().await.transaction())
                .await
                .unwrap();
        }
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // WHEN
        let fetched_groups = list_firewall_group(&project.id, connection_pool())
            .await
            .unwrap();

        // THEN
        assert_eq!(fetched_groups.len(), 2);
        assert!(fetched_groups
            .iter()
            .any(|g| g.description == "test-group-1"));
        assert!(fetched_groups
            .iter()
            .any(|g| g.description == "test-group-2"));
    }

    #[tokio::test]
    async fn test_list_object_storage() {
        // GIVEN
        tear_down().await;
        let (_, project) = create_project_helper().await;
        let ext = SqlExecutor::new();
        ext.write().await.begin().await.unwrap();

        // Create multiple object storages
        let object_storages = vec![
            ObjectStorage {
                project_id: project.id,
                tier_id: Some(1),
                id: Uuid::new_v4(),
                cluster_id: 1,
                label: "test-storage-1".to_string(),
                x: 0,
                y: 0,
            },
            ObjectStorage {
                project_id: project.id,
                tier_id: Some(2),
                id: Uuid::new_v4(),
                cluster_id: 2,
                label: "test-storage-2".to_string(),
                x: 0,
                y: 0,
            },
        ];

        for storage in &object_storages {
            insert_object_storage(storage, ext.write().await.transaction())
                .await
                .unwrap();
        }
        ext.write().await.commit().await.unwrap();
        ext.write().await.close().await;

        // WHEN
        let fetched_storages = list_object_storage(&project.id, connection_pool())
            .await
            .unwrap();

        // THEN
        assert_eq!(fetched_storages.len(), 2);
        assert!(fetched_storages
            .iter()
            .any(|s| s.label == "test-storage-1" && s.tier_id.unwrap() == 1));
        assert!(fetched_storages
            .iter()
            .any(|s| s.label == "test-storage-2" && s.tier_id.unwrap() == 2));
    }
}
