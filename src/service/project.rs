use crate::adapter::kv_store::interfaces::{KVStore, VultrKeyPairStore};
use crate::adapter::kv_store::rocks_db::get_rocks_db;
use crate::adapter::mail::{send_email, Email, EmailType};
use crate::adapter::repositories::interfaces::TExecutor;
use crate::adapter::repositories::project::diagram::{
    list_block_storage, list_compute, list_firewall_group, list_firewall_rule,
    list_managed_database, list_object_storage,
};
use crate::adapter::repositories::project::workspace::{
    delete_project, delete_user_role, get_project, get_user_role, get_vult_api_key, insert_project,
    upsert_user_role, upsert_vult_api_key,
};
use crate::adapter::repositories::{connection_pool, SqlExecutor};
use crate::adapter::request_dispensor::architector_server::{
    request_architecture_recommendation, ArchitectureRecommendation, RequestArchitectureSuggestion,
};
use crate::domain::project::commands::{
    AssignRole, DeleteProject, DeployProject, ExpelMember, RegisterVultApiKey, ResourceResponse,
    VultrCommand,
};
use crate::domain::project::diagrams::{get_diagram_key, get_diagram_update_dt};
use crate::domain::project::enums::ResourceType;
use crate::domain::project::{commands::CreateProject, ProjectAggregate};
use crate::domain::project::{UserRole, UserRoleEntity, VultApiKeyEntity, VultrCommandManager};
use crate::errors::ServiceError;
use crate::CurrentUser;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn handle_create_project(
    cmd: CreateProject,
    current_user: CurrentUser,
) -> Result<Uuid, ServiceError> {
    let ext = SqlExecutor::new();
    ext.write().await.begin().await?;

    let project = ProjectAggregate::new(cmd.name, cmd.description);
    let user_role = UserRoleEntity::new(project.id, current_user.email, UserRole::Admin);

    insert_project(&project, ext.write().await.transaction()).await?;
    upsert_user_role(&user_role, ext.write().await.transaction()).await?;

    ext.write().await.commit().await?;
    ext.write().await.close().await;
    Ok(project.id)
}

pub async fn handle_delete_project(cmd: DeleteProject) -> Result<(), ServiceError> {
    let ext = SqlExecutor::new();
    ext.write().await.begin().await?;

    delete_project(cmd.project_id, ext.write().await.transaction()).await?;

    ext.write().await.commit().await?;
    ext.write().await.close().await;
    Ok(())
}

pub async fn handle_assign_role(
    cmd: AssignRole,
    current_user: CurrentUser,
) -> Result<(), ServiceError> {
    let ext = SqlExecutor::new();

    // * User who is inviting must be admin
    let inviter_role = get_user_role(cmd.project_id, &current_user.email, connection_pool())
        .await
        .map_err(|_| ServiceError::Unauthorized)?;
    let project = get_project(cmd.project_id, connection_pool()).await?;
    inviter_role.verify_role(&[UserRole::Admin])?;

    let invitee_role = UserRoleEntity::new(cmd.project_id, cmd.invitee_email.clone(), cmd.role);

    ext.write().await.begin().await?;
    upsert_user_role(&invitee_role, ext.write().await.transaction()).await?;
    ext.write().await.commit().await?;
    ext.write().await.close().await;

    let email = Email::new(cmd.invitee_email, EmailType::ProjectInvitation(&project.id));
    send_email(email).await?;
    Ok(())
}

pub async fn handle_expel_member(
    cmd: ExpelMember,
    current_user: CurrentUser,
) -> Result<(), ServiceError> {
    let ext = SqlExecutor::new();
    let user_role = get_user_role(cmd.project_id, &current_user.email, connection_pool())
        .await
        .map_err(|_| ServiceError::Unauthorized)?;
    user_role.verify_role(&[UserRole::Admin])?;
    ext.write().await.begin().await?;

    delete_user_role(
        cmd.project_id,
        &cmd.expelled_email,
        ext.write().await.transaction(),
    )
    .await?;
    ext.write().await.commit().await?;
    ext.write().await.close().await;
    Ok(())
}

pub async fn handle_get_public_key() -> Result<String, ServiceError> {
    let rocks_db = get_rocks_db().await;
    let public_key = rocks_db.get_or_create_public_key().await?;

    String::from_utf8(public_key.key.public_key_to_pem()?).map_err(|_| ServiceError::ParseError)
}

pub async fn handle_register_vultr_api_key(
    cmd: RegisterVultApiKey,
    current_user: CurrentUser,
) -> Result<(), ServiceError> {
    let user_role = get_user_role(cmd.project_id, &current_user.email, connection_pool()).await?;
    user_role.verify_role(&[UserRole::Admin])?;
    let ext = SqlExecutor::new();
    ext.write().await.begin().await?;
    // let rocks_db = get_rocks_db().await;

    // let private_key = rocks_db.get(RocksDB::PRIVATE_KEY_NAME).await?;
    // let private_key = PrivateKey::from_pem(&private_key)?;
    // let api_key = private_key.decode_data(&cmd.api_key)?;

    upsert_vult_api_key(
        &VultApiKeyEntity::new(cmd.project_id, cmd.api_key),
        ext.write().await.transaction(),
    )
    .await?;
    ext.write().await.commit().await?;
    ext.write().await.close().await;
    Ok(())
}

pub async fn handle_session_sse(
    current_user: &CurrentUser,
    project_id: Uuid,
    last_update_dt: &mut DateTime<Utc>,
) -> Result<Option<Value>, ServiceError> {
    let rocks_db = get_rocks_db().await;
    let _ = get_user_role(project_id, &current_user.email, connection_pool()).await?; // To check if the user is a member of the project

    let key = get_diagram_update_dt(project_id);
    let update_dt = rocks_db.get(key.as_bytes()).await?;
    let update_dt =
        String::from_utf8(update_dt).map_err(|err| ServiceError::ParsingError(Box::new(err)))?;
    let update_dt = DateTime::parse_from_rfc3339(&update_dt)
        .map_err(|err| ServiceError::ParsingError(Box::new(err)))?
        .with_timezone(&Utc);
    if update_dt > *last_update_dt {
        *last_update_dt = update_dt;
        let key = get_diagram_key(project_id);
        let res_bytes = rocks_db.get(key.as_bytes()).await?;
        let res = serde_json::from_slice::<Vec<ResourceResponse>>(&res_bytes)?;
        Ok(Some(json!(res)))
    } else {
        Ok(None)
    }
}

pub async fn handle_deploy_project(
    cmd: DeployProject,
    current_user: CurrentUser,
) -> Result<(), ServiceError> {
    let user_role = get_user_role(cmd.project_id, &current_user.email, connection_pool()).await?;
    user_role.verify_role(&[UserRole::Admin, UserRole::Editor])?;
    let vultr_api_key = get_vult_api_key(cmd.project_id, connection_pool()).await?;
    if vultr_api_key.api_key.is_empty() {
        return Err(ServiceError::NotFound);
    }

    let ext = SqlExecutor::new();
    ext.write().await.begin().await?;
    let mut trx = ext.write().await;
    let command_list = cmd
        .command_list
        .into_iter()
        .map(VultrCommand::from_request)
        .collect::<Result<Vec<_>, _>>()?;
    let mut vultr_command_manager = VultrCommandManager::new(
        command_list,
        cmd.project_id,
        vultr_api_key.api_key,
        trx.transaction(),
    );
    match vultr_command_manager.execute().await {
        Ok(_) => {
            trx.commit().await?;
        }
        Err(e) => {
            trx.rollback().await?;
            return Err(e);
        }
    }
    trx.close().await;

    let res = update_project_diagram(cmd.project_id).await?;
    let rocks_db = get_rocks_db().await;
    let res_bytes = serde_json::to_vec(&res)?;
    let key = get_diagram_update_dt(cmd.project_id);
    rocks_db
        .insert(key.as_bytes(), Utc::now().to_rfc3339().as_bytes())
        .await?;

    let key = get_diagram_key(cmd.project_id);
    rocks_db.insert(key.as_bytes(), &res_bytes).await?;

    Ok(())
}

async fn update_project_diagram(project_id: Uuid) -> Result<Vec<ResourceResponse>, ServiceError> {
    let mut res: Vec<ResourceResponse> = Vec::new();
    let conn = connection_pool();
    let (
        mut compute_list,
        mut managed_database,
        mut object_storage,
        mut block_storage,
        mut firewall_group,
        mut firewall_rule,
    ) = tokio::try_join!(
        list_compute(&project_id, conn),
        list_managed_database(&project_id, conn),
        list_object_storage(&project_id, conn),
        list_block_storage(&project_id, conn),
        list_firewall_group(&project_id, conn),
        list_firewall_rule(&project_id, conn),
    )?;
    compute_list.iter_mut().for_each(|compute| {
        res.push(ResourceResponse::into_response(ResourceType::Compute, json!(compute)).unwrap());
    });
    managed_database.iter_mut().for_each(|managed_database| {
        res.push(
            ResourceResponse::into_response(ResourceType::ManagedDatabase, json!(managed_database))
                .unwrap(),
        );
    });
    object_storage.iter_mut().for_each(|object_storage| {
        res.push(
            ResourceResponse::into_response(ResourceType::ObjectStorage, json!(object_storage))
                .unwrap(),
        );
    });
    block_storage.iter_mut().for_each(|block_storage| {
        res.push(
            ResourceResponse::into_response(ResourceType::BlockStorage, json!(block_storage))
                .unwrap(),
        );
    });
    firewall_group.iter_mut().for_each(|firewall_group| {
        res.push(
            ResourceResponse::into_response(ResourceType::FirewallGroup, json!(firewall_group))
                .unwrap(),
        );
    });
    firewall_rule.iter_mut().for_each(|firewall_rule| {
        res.push(
            ResourceResponse::into_response(ResourceType::FirewallRule, json!(firewall_rule))
                .unwrap(),
        );
    });
    Ok(res)
}

pub async fn handle_request_architecture_suggestion(
    cmd: RequestArchitectureSuggestion,
    current_user: CurrentUser,
    project_id: Uuid,
) -> Result<ArchitectureRecommendation, ServiceError> {
    let user_role = get_user_role(project_id, &current_user.email, connection_pool()).await?;
    user_role.verify_role(&[UserRole::Admin, UserRole::Editor])?;
    tracing::info!("Waiting for architecture recommendation...\nproject_id: {project_id}");
    request_architecture_recommendation(cmd).await
}

#[cfg(test)]
mod tests {
    use openssl::rsa::Padding;

    use super::*;
    use crate::{
        adapter::repositories::{
            connection_pool,
            project::workspace::{get_project, get_user_role},
        },
        domain::auth::{commands::CreateUserAccount, private_key::PublicKey, UserAccountAggregate},
        service::auth::tests::create_user_account_helper,
    };

    pub async fn create_project_helper() -> (UserAccountAggregate, ProjectAggregate, CurrentUser) {
        let user_account = create_user_account_helper().await;
        let create_project_cmd = CreateProject {
            name: "test".to_string(),
            description: "test".to_string(),
        };
        let current_user = CurrentUser {
            email: user_account.email.clone(),
        };
        let project_id = handle_create_project(create_project_cmd.clone(), current_user.clone())
            .await
            .unwrap();
        let project = get_project(project_id, connection_pool()).await.unwrap();
        (user_account, project, current_user)
    }

    #[tokio::test]
    async fn test_create_project() {
        // GIVEN
        let create_user_cmd = CreateUserAccount {
            email: format!("{}@test.com", Uuid::new_v4()),
            password: format!("password{}", Uuid::new_v4()),
            name: "test".to_string(),
            phone_num: "01012345678".to_string(),
        };
        let create_project_cmd = CreateProject {
            name: "test".to_string(),
            description: "test".to_string(),
        };

        // WHEN
        let current_user = CurrentUser {
            email: create_user_cmd.email.clone(),
        };
        let project_id = handle_create_project(create_project_cmd, current_user.clone())
            .await
            .unwrap();

        // THEN
        let project = get_project(project_id, connection_pool()).await.unwrap();
        let user_role = get_user_role(project_id, &create_user_cmd.email, connection_pool())
            .await
            .unwrap();
        assert_eq!(project.name, "test");
        assert_eq!(project.description, "test");
        assert_eq!(project.id, project_id);
        assert_eq!(user_role.role, UserRole::Admin);
        assert_eq!(user_role.project_id, project_id);
        assert_eq!(user_role.user_email, create_user_cmd.email);
    }

    #[tokio::test]
    async fn test_delete_project() {
        // GIVEN
        let (user_account, project, _) = create_project_helper().await;
        let delete_project_cmd = DeleteProject {
            project_id: project.id,
        };

        // WHEN
        handle_delete_project(delete_project_cmd).await.unwrap();

        // THEN
        assert!(matches!(
            get_project(project.id, connection_pool()).await,
            Err(ServiceError::NotFound)
        ));
        assert!(matches!(
            get_user_role(project.id, &user_account.email, connection_pool()).await,
            Err(ServiceError::NotFound)
        ));
    }

    #[tokio::test]
    async fn test_assign_role() {
        // GIVEN
        let (user_account, project, current_user) = create_project_helper().await;
        let assign_role_cmd = AssignRole {
            project_id: project.id,
            invitee_email: user_account.email.clone(),
            role: UserRole::Editor,
        };
        handle_assign_role(assign_role_cmd, current_user.clone())
            .await
            .unwrap();

        // THEN
        let user_role = get_user_role(project.id, &user_account.email, connection_pool())
            .await
            .unwrap();
        assert_eq!(user_role.role, UserRole::Editor);
    }

    #[tokio::test]
    async fn test_expel_member() {
        // GIVEN
        let (admin_user_account, project, current_user) = create_project_helper().await;
        let expel_member_cmd = ExpelMember {
            project_id: project.id,
            expelled_email: admin_user_account.email.clone(),
        };

        // WHEN
        let user_role = get_user_role(project.id, &admin_user_account.email, connection_pool())
            .await
            .unwrap();
        user_role.verify_role(&[UserRole::Admin]).unwrap();
        handle_expel_member(expel_member_cmd, current_user.clone())
            .await
            .unwrap();

        // THEN
        assert!(matches!(
            get_user_role(project.id, &admin_user_account.email, connection_pool()).await,
            Err(ServiceError::NotFound)
        ));
    }
    #[tokio::test]
    async fn test_assign_role_by_non_admin() {
        // GIVEN
        let (_, project, _) = create_project_helper().await;
        let non_admin_user_account = create_user_account_helper().await;
        let assign_role_cmd = AssignRole {
            project_id: project.id,
            invitee_email: non_admin_user_account.email.clone(),
            role: UserRole::Viewer,
        };
        let current_user = CurrentUser {
            email: non_admin_user_account.email.clone(),
            // user_id: non_admin_user_account.id,
        };

        // THEN
        assert!(matches!(
            handle_assign_role(assign_role_cmd, current_user.clone()).await,
            Err(ServiceError::Unauthorized)
        ));
    }
    #[tokio::test]
    async fn test_expel_member_by_non_admin() {
        // GIVEN
        let (_, project, current_user) = create_project_helper().await;
        let non_admin_user_account = create_user_account_helper().await;
        let assign_role_cmd = AssignRole {
            project_id: project.id,
            invitee_email: non_admin_user_account.email.clone(),
            role: UserRole::Editor,
        };
        handle_assign_role(assign_role_cmd, current_user.clone())
            .await
            .unwrap();

        // WHEN
        let non_admin_user = CurrentUser {
            email: non_admin_user_account.email.clone(),
            // user_id: non_admin_user_account.id,
        };
        let expel_member_cmd = ExpelMember {
            project_id: project.id,
            expelled_email: non_admin_user_account.email.clone(),
        };

        // THEN
        assert!(matches!(
            handle_expel_member(expel_member_cmd, non_admin_user.clone()).await,
            Err(ServiceError::Unauthorized)
        ));
    }
    #[tokio::test]
    async fn test_get_public_key() {
        // GIVEN
        let rocks_db = get_rocks_db().await;
        let tmp_api_key = "tmp_api_key";

        let public_key_1 = handle_get_public_key().await.unwrap();
        assert!(!public_key_1.is_empty());

        // WHEN
        let public_key = PublicKey::from_pem(&public_key_1.as_bytes()).unwrap();
        let private_key = crate::domain::auth::private_key::PrivateKey::from_pem(
            &rocks_db
                .get(crate::adapter::kv_store::rocks_db::RocksDB::PRIVATE_KEY_NAME)
                .await
                .unwrap(),
        )
        .unwrap();
        let mut buf: Vec<u8> = vec![0; public_key.key.size() as usize];
        let token_len = public_key
            .key
            .public_encrypt(tmp_api_key.as_bytes(), &mut buf, Padding::PKCS1)
            .unwrap();

        let public_key_2 = handle_get_public_key().await.unwrap();
        assert!(!public_key_2.is_empty());

        // THEN
        let decoded_token = private_key.decode_data(&buf[0..token_len]).unwrap();
        assert_eq!(public_key_1, public_key_2);
        assert_eq!(decoded_token, tmp_api_key);
    }
    // #[tokio::test]
    // async fn test_register_vult_api_key() {
    //     // GIVEN
    //     let rocks_db = get_rocks_db().await;
    //     rocks_db.delete(RocksDB::PRIVATE_KEY_NAME).await.unwrap();
    //     rocks_db.delete(RocksDB::PUBLIC_KEY_NAME).await.unwrap();

    //     let (_, project, current_user) = create_project_helper().await;
    //     let public_key = handle_get_public_key().await.unwrap();
    //     let test_api_key = "test api_key";

    //     let encoded_api_key = encode_data_with_public_key(
    //         PublicKey::from_pem(public_key.as_bytes()).unwrap(),
    //         test_api_key.as_bytes(),
    //     )
    //     .await;
    //     let register_vult_api_key_cmd = RegisterVultApiKey {
    //         project_id: project.id,
    //         api_key: encoded_api_key,
    //     };

    //     // WHEN
    //     handle_register_vultr_api_key(register_vult_api_key_cmd, current_user.clone())
    //         .await
    //         .unwrap();

    //     // THEN
    //     let vult_api_key = get_vult_api_key(project.id, connection_pool())
    //         .await
    //         .unwrap();
    //     assert_eq!(vult_api_key.api_key, test_api_key);
    // }

    // #[tokio::test]
    // async fn test_register_vult_api_key_by_non_admin() {
    //     // GIVEN
    //     let rocks_db = get_rocks_db().await;
    //     rocks_db.delete(RocksDB::PRIVATE_KEY_NAME).await.unwrap();
    //     rocks_db.delete(RocksDB::PUBLIC_KEY_NAME).await.unwrap();

    //     let (_, project, current_user) = create_project_helper().await;
    //     let public_key = handle_get_public_key().await.unwrap();
    //     let test_api_key = "test api_key";

    //     let encoded_api_key = encode_data_with_public_key(
    //         PublicKey::from_pem(public_key.as_bytes()).unwrap(),
    //         test_api_key.as_bytes(),
    //     )
    //     .await;
    //     let register_vult_api_key_cmd = RegisterVultApiKey {
    //         project_id: project.id,
    //         api_key: encoded_api_key,
    //     };

    //     let mut user_role = get_user_role(project.id, &current_user.email, connection_pool())
    //         .await
    //         .unwrap();
    //     user_role.role = UserRole::Viewer;

    //     let ext = SqlExecutor::new();
    //     ext.write().await.begin().await.unwrap();
    //     upsert_user_role(&user_role, ext.write().await.transaction())
    //         .await
    //         .unwrap();

    //     ext.write().await.commit().await.unwrap();
    //     ext.write().await.close().await;

    //     // WHEN
    //     let result =
    //         handle_register_vultr_api_key(register_vult_api_key_cmd, current_user.clone()).await;

    //     // THEN
    //     assert!(matches!(result, Err(ServiceError::Unauthorized)));
    // }
}
