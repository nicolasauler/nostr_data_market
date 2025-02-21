use crate::rest::user::SignUpInput;
use sqlx::PgPool;
use time::OffsetDateTime;

pub async fn create(db_pool: PgPool, create_user: SignUpInput) -> sqlx::Result<()> {
    let now = OffsetDateTime::now_utc();
    crate::queries::user::create(
        &db_pool,
        create_user.username.as_str(),
        create_user.pubkey.as_str(),
        now,
    )
    .await
    .map(|c| {
        if c.rows_affected() > 1 {
            tracing::error!("i really need a macro that cancels the transaction");
        }
    })
}
