use sqlx::PgExecutor;
use time::OffsetDateTime;

pub async fn create(
    conn: impl PgExecutor<'_>,
    username: &str,
    pubkey: &str,
    created_at: OffsetDateTime,
) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
    sqlx::query!(
        r#"
        INSERT INTO users (username, created_at, pubkey)
        VALUES ($1, $2, $3)
        "#,
        username,
        created_at,
        pubkey,
    )
    .execute(conn)
    .await
}
