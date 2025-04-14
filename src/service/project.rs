use crate::adapter::mail::{send_email, Email, EmailType};
use crate::adapter::repositories::interfaces::TExecutor;
use crate::adapter::repositories::project::{
    delete_project, delete_user_role, get_project, get_user_role, insert_project, upsert_user_role,
};
use crate::adapter::repositories::{connection_pool, SqlExecutor};
use crate::domain::project::commands::{AssignRole, DeleteProject, ExpelMember};
use crate::domain::project::{commands::CreateProject, ProjectAggregate};
use crate::domain::project::{UserRole, UserRoleEntity};
use crate::errors::ServiceError;
use crate::CurrentUser;
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
    inviter_role.verify_admin()?;

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
    user_role.verify_admin()?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adapter::repositories::{
            connection_pool,
            project::{get_project, get_user_role},
        },
        domain::auth::{commands::CreateUserAccount, UserAccountAggregate},
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
            Err(ServiceError::RowNotFound)
        ));
        assert!(matches!(
            get_user_role(project.id, &user_account.email, connection_pool()).await,
            Err(ServiceError::RowNotFound)
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
        user_role.verify_admin().unwrap();
        handle_expel_member(expel_member_cmd, current_user.clone())
            .await
            .unwrap();

        // THEN
        assert!(matches!(
            get_user_role(project.id, &admin_user_account.email, connection_pool()).await,
            Err(ServiceError::RowNotFound)
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
}
