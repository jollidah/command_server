use super::routes::auth;
use crate::adapter::http::schemas::UserAccount;
use utoipa::OpenApi;

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
