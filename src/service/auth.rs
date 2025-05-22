use uuid::Uuid;

use crate::{
    adapter::{
        kv_store::{interfaces::KVStore, rocks_db::get_rocks_db},
        mail::{send_email, Email, EmailType},
        repositories::{
            auth::{get_user_account_by_email, insert_user_account, update_user_account},
            connection_pool,
            interfaces::TExecutor,
            SqlExecutor,
        },
    },
    domain::auth::{
        commands::{CheckVerification, CreateUserAccount, IssueTokens, RefreshTokens},
        jwt::JwtToken,
        AuthenticationTokens, UserAccountAggregate, VerificationCode,
    },
    errors::ServiceError,
};
// TODO refactor to use repository instead of executor
pub async fn handle_create_user_account(command: CreateUserAccount) -> Result<Uuid, ServiceError> {
    let ext = SqlExecutor::new();
    ext.write().await.begin().await?;

    let code = VerificationCode::new();
    let email = Email::new(
        command.email.clone(),
        EmailType::VerificationCode(&code.code),
    );
    let user = UserAccountAggregate::from(command);

    insert_user_account(&user, ext.write().await.transaction()).await?;
    ext.write().await.commit().await?;

    // Insert Data to KVStore
    let rocks_db = get_rocks_db().await;
    rocks_db
        .insert(user.email.as_bytes(), &code.to_bytes().unwrap())
        .await?;

    // Send Email
    send_email(email).await?;
    Ok(user.id)
}

pub async fn handle_check_verification_email(
    command: CheckVerification,
) -> Result<(), ServiceError> {
    let ext = SqlExecutor::new();
    let mut user = get_user_account_by_email(&command.email, connection_pool()).await?;
    ext.write().await.begin().await?;

    let rocks_db = get_rocks_db().await;
    let code = VerificationCode::from_bytes(&rocks_db.pop(command.email.as_bytes()).await?)?;
    code.verify_code(&command.verification_code)?;

    user.set_account_verified();

    update_user_account(&user, ext.write().await.transaction()).await?;
    ext.write().await.commit().await?;
    Ok(())
}

pub async fn handle_issue_tokens(
    command: IssueTokens,
) -> Result<AuthenticationTokens, ServiceError> {
    let ext = SqlExecutor::new();
    let user = get_user_account_by_email(&command.email, connection_pool()).await?;
    if !user.verified {
        return Err(ServiceError::UserNotVerified);
    } else if user.password != command.password {
        return Err(ServiceError::Unauthorized);
    }
    ext.write().await.begin().await?;

    let jwt = JwtToken::new();
    let access_token = jwt.generate_access_token(user.id, &command.email)?;
    let refresh_token = jwt.generate_refresh_token(user.id, &command.email)?;

    let tokens = AuthenticationTokens::new(access_token, refresh_token);

    Ok(tokens)
}

pub async fn handle_refresh_tokens(
    command: RefreshTokens,
) -> Result<AuthenticationTokens, ServiceError> {
    let ext = SqlExecutor::new();
    ext.write().await.begin().await?;
    let jwt = JwtToken::new();
    let claims = jwt.verify_token(&command.refresh_token)?;
    let access_token = jwt.generate_access_token(claims.user_id, &claims.email)?;

    let tokens = AuthenticationTokens::new(access_token, command.refresh_token);

    Ok(tokens)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    pub async fn create_user_account_helper() -> UserAccountAggregate {
        let cmd = CreateUserAccount {
            email: format!("{}@test.com", Uuid::new_v4()),
            password: format!("password{}", Uuid::new_v4()),
            name: "Test User".to_string(),
            phone_num: "01012345678".to_string(),
        };
        handle_create_user_account(cmd.clone()).await.unwrap();
        get_user_account_by_email(&cmd.email, connection_pool())
            .await
            .unwrap()
    }
    #[tokio::test]
    async fn test_create_user_account() {
        // GIVEN
        let cmd = CreateUserAccount {
            email: format!("{}@test.com", Uuid::new_v4()),
            password: format!("password{}", Uuid::new_v4()),
            name: "Test User".to_string(),
            phone_num: "01012345678".to_string(),
        };

        // WHEN
        handle_create_user_account(cmd.clone()).await.unwrap();
        let user = get_user_account_by_email(&cmd.email, connection_pool())
            .await
            .unwrap();

        // THEN
        assert_eq!(user.email, cmd.email);
        assert_eq!(user.name, cmd.name);
        assert_eq!(user.phone_num, cmd.phone_num);
        assert_eq!(user.password, cmd.password);
        assert_eq!(user.verified, false);
        let rocks_db = get_rocks_db().await;
        let verfication_code =
            VerificationCode::from_bytes(&rocks_db.get(&cmd.email.as_bytes()).await.unwrap())
                .unwrap();
        assert!(verfication_code.expires_at > Utc::now());
    }

    #[tokio::test]
    async fn test_check_verification_email() {
        // GIVEN
        let rocks_db = get_rocks_db().await;
        let user_account = create_user_account_helper().await;
        let verfication_code = VerificationCode::from_bytes(
            &rocks_db.get(&user_account.email.as_bytes()).await.unwrap(),
        )
        .unwrap();

        let cmd = CheckVerification {
            email: user_account.email.clone(),
            verification_code: verfication_code.code.clone(),
        };

        // WHEN
        handle_check_verification_email(cmd.clone()).await.unwrap();

        // THEN
        assert!(matches!(
            rocks_db.get(&cmd.email.as_bytes()).await.unwrap_err(),
            ServiceError::NotFound
        ));
        let user = get_user_account_by_email(&cmd.email, connection_pool())
            .await
            .unwrap();
        assert_eq!(user.verified, true);
    }

    #[tokio::test]
    async fn test_check_verification_email_expired() {
        // GIVEN
        let rocks_db = get_rocks_db().await;
        let user_account = create_user_account_helper().await;
        let mut verfication_code = VerificationCode::from_bytes(
            &rocks_db.pop(&user_account.email.as_bytes()).await.unwrap(),
        )
        .unwrap();
        verfication_code.expires_at = Utc::now() - Duration::minutes(1);
        rocks_db
            .insert(
                &user_account.email.as_bytes(),
                &verfication_code.to_bytes().unwrap(),
            )
            .await
            .unwrap();

        let cmd = CheckVerification {
            email: user_account.email.clone(),
            verification_code: verfication_code.code.clone(),
        };

        // THEN
        assert!(matches!(
            handle_check_verification_email(cmd.clone()).await,
            Err(ServiceError::VerificationCodeExpired)
        ));
    }

    #[tokio::test]
    async fn test_issue_tokens() {
        // GIVEN
        let rocks_db = get_rocks_db().await;
        let user_account = create_user_account_helper().await;
        let issue_tokens_cmd = IssueTokens {
            email: user_account.email.clone(),
            password: user_account.password.clone(),
        };
        let user = get_user_account_by_email(&user_account.email, connection_pool())
            .await
            .unwrap();
        let verfication_code = VerificationCode::from_bytes(
            &rocks_db.get(&user_account.email.as_bytes()).await.unwrap(),
        )
        .unwrap();

        let check_verification_cmd = CheckVerification {
            email: user_account.email.clone(),
            verification_code: verfication_code.code.clone(),
        };

        // WHEN
        assert!(matches!(
            handle_issue_tokens(issue_tokens_cmd.clone()).await,
            Err(ServiceError::UserNotVerified)
        ));
        handle_check_verification_email(check_verification_cmd.clone())
            .await
            .unwrap();
        let authentication_tokens = handle_issue_tokens(issue_tokens_cmd.clone()).await.unwrap();

        // THEN
        let jwt = JwtToken::new();
        let claims = jwt
            .verify_token(&authentication_tokens.refresh_token)
            .unwrap();
        assert_eq!(claims.email, issue_tokens_cmd.email);
        assert!(claims.user_id == user.id);
    }

    #[tokio::test]
    async fn test_refresh_tokens() {
        // GIVEN

        // 1. Create user account and get initial tokens
        let user_account = create_user_account_helper().await;

        // Verify email
        let rocks_db = get_rocks_db().await;
        let verfication_code = VerificationCode::from_bytes(
            &rocks_db.get(&user_account.email.as_bytes()).await.unwrap(),
        )
        .unwrap();
        let verify_cmd = CheckVerification {
            email: user_account.email.clone(),
            verification_code: verfication_code.code.clone(),
        };
        handle_check_verification_email(verify_cmd).await.unwrap();

        // Get initial tokens
        let issue_cmd = IssueTokens {
            email: user_account.email.clone(),
            password: user_account.password.clone(),
        };

        // WHEN
        let initial_tokens = handle_issue_tokens(issue_cmd).await.unwrap();

        // 2. Verify initial tokens are valid
        let jwt = JwtToken::new();
        let initial_access_claims = jwt.verify_token(&initial_tokens.access_token).unwrap();
        let initial_refresh_claims = jwt.verify_token(&initial_tokens.refresh_token).unwrap();

        assert_eq!(initial_access_claims.email, user_account.email);
        assert_eq!(initial_refresh_claims.email, user_account.email);

        // 3. Use refresh token to get new tokens
        let refresh_cmd = RefreshTokens {
            refresh_token: initial_tokens.refresh_token.clone(),
        };
        let new_tokens = handle_refresh_tokens(refresh_cmd).await.unwrap();

        // THEN
        let new_access_claims = jwt.verify_token(&new_tokens.access_token).unwrap();
        let new_refresh_claims = jwt.verify_token(&new_tokens.refresh_token).unwrap();
        assert_eq!(new_access_claims.email, user_account.email);
        assert_eq!(new_refresh_claims.email, user_account.email);
        assert_eq!(new_access_claims.user_id, initial_access_claims.user_id);
        assert_eq!(new_refresh_claims.user_id, initial_refresh_claims.user_id);
        assert!(new_access_claims.exp > Utc::now().timestamp());
        assert_eq!(new_tokens.refresh_token, initial_tokens.refresh_token);
    }
}
