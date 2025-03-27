use utoipa::OpenApi;
use super::routes::auth;
use crate::adapter::http::schemas::UserAccount;

#[derive(OpenApi)]
#[openapi(
 paths(
    auth::test_auth,
 ),
 components(
     schemas(
        UserAccount,
      )
 ),
 tags(
    (name= "Authentication Management Service", description="Authenticate end-user and manage user information")
    )
 )]
pub struct AuthDoc;
