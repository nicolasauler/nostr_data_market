use sqlx::PgPool;
use time::OffsetDateTime;

pub async fn exists(db: PgPool, pubkey: &str) -> sqlx::Result<bool> {
    crate::queries::user::exists(&db, pubkey)
        .await
        .map(|r| r.unwrap_or(false))
}

pub async fn create(db_pool: PgPool, pubkey: &str, nickname: &str) -> sqlx::Result<()> {
    let now = OffsetDateTime::now_utc();
    crate::queries::user::create(&db_pool, pubkey, nickname, now)
        .await
        .map(|c| {
            if c.rows_affected() > 1 {
                tracing::error!("i really need a macro that cancels the transaction");
            }
        })
}

pub async fn find(db_pool: PgPool, pubkey: &str) -> sqlx::Result<Option<String>> {
    crate::queries::user::find(&db_pool, pubkey)
        .await
        .map(|r| r.map(|r| r.username))
}
