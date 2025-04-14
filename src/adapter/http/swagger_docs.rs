use crate::domain::{
    auth::{
        commands::{CheckVerification, CreateUserAccount, IssueTokens, RefreshTokens},
        AuthenticationTokens,
    },
    project::{
        commands::{AssignRole, CreateProject, DeleteProject, ExpelMember},
        UserRole,
    },
};

use super::routes::{auth, project};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
 paths(
    auth::create_user_account,
    auth::issue_tokens,
    auth::check_verification_email,
    auth::refresh_tokens,
 ),
 components(
     schemas(
         CreateUserAccount,
         IssueTokens,
         CheckVerification,
         RefreshTokens,
         AuthenticationTokens,
      )
 ),
 tags(
    (name= "Authentication Management Service", description="Authenticate end-user and manage user information")
    )
 )]
pub struct AuthDoc;

#[derive(OpenApi)]
#[openapi(
 paths(
    project::assign_role,
    project::expel_member,
    project::create_project,
    project::delete_project,
 ),
 components(
     schemas(
         AssignRole,
         ExpelMember,
         CreateProject,
         DeleteProject,
         UserRole,
      )
 ),
 tags(
    (name= "Project Management Service", description="Manage project")
    )
 )]
pub struct ProjectDoc;
