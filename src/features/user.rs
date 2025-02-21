use crate::rest::user::LoginInput;
use sqlx::PgPool;
use time::OffsetDateTime;

pub async fn find(db: PgPool, pubkey: &str) -> sqlx::Result<bool> {
    crate::queries::user::exists(&db, pubkey)
        .await
        .map(|r| r.unwrap_or(false))
}

//pub async fn create(db_pool: PgPool, create_user: LoginInput) -> sqlx::Result<()> {
//let now = OffsetDateTime::now_utc();
//crate::queries::user::create(&db_pool, create_user.pubkey.as_str(), now)
//.await
//.map(|c| {
//if c.rows_affected() > 1 {
//tracing::error!("i really need a macro that cancels the transaction");
//}
//})
//}
