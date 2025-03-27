use sqlx::query_as;

use crate::adapter::http::schemas::UserAccount;

pub async fn _run_dummy_queries(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // SELECT
    let _ = query_as!(
        UserAccount,
        r#"
        SELECT id, email, name, phone_num, verified, create_dt
        FROM account_user
        LIMIT 1
        "#
    )
    .fetch_optional(pool)
    .await?;

    Ok(())
}
