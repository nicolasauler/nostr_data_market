use sqlx::PgExecutor;
use time::OffsetDateTime;

pub async fn create(
    conn: impl PgExecutor<'_>,
    pubkey: &str,
    username: &str,
    created_at: OffsetDateTime,
) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
    sqlx::query!(
        r#"
        INSERT INTO users (pubkey, username, created_at)
        VALUES ($1, $2, $3)
        "#,
        pubkey,
        username,
        created_at,
    )
    .execute(conn)
    .await
}

pub struct Pubkey {
    pub pubkey: String,
}

pub async fn list_pubkeys(conn: impl PgExecutor<'_>) -> sqlx::Result<Vec<Pubkey>> {
    sqlx::query_as!(
        Pubkey,
        r#"SELECT pubkey
        FROM users"#,
    )
    .fetch_all(conn)
    .await
}

pub async fn exists(conn: impl PgExecutor<'_>, pubkey: &str) -> Result<Option<bool>, sqlx::Error> {
    let record = sqlx::query!(
        r#"
        select exists(select 1 from users where pubkey = $1)
        "#,
        pubkey
    )
    .fetch_one(conn)
    .await?;

    Ok(record.exists)
}
