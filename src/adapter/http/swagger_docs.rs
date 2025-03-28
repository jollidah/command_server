use crate::{adapter::http::schemas::{CheckVerification, CreateProject, CreateVerification, DeleteProject, ExpelMember, SignIn, SignUp}, domain::project::AssignRole};

use super::routes::{auth, project};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
 paths(
    auth::sign_up,
    auth::sign_in,
    auth::create_verification_email,
    auth::check_verification_email,
 ),
 components(
     schemas(
         SignUp,
         SignIn,
         CreateVerification,
         CheckVerification,
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
      )
 ),
 tags(
    (name= "Project Management Service", description="Manage project")
    )
 )]
pub struct ProjectDoc;
