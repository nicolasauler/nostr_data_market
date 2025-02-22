use sqlx::PgExecutor;

pub async fn create(
    conn: impl PgExecutor<'_>,
    pubkey: &str,
    sensor_id: &str,
    description: &str,
    now: time::OffsetDateTime,
) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
    sqlx::query!(
        r#"
        INSERT INTO user_sensors (user_pubkey, external_id, description, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        pubkey,
        sensor_id,
        description,
        now,
    )
    .execute(conn)
    .await
}

pub struct Sensor {
    pub external_id: String,
    pub description: String,
}

pub async fn list(conn: impl PgExecutor<'_>) -> sqlx::Result<Vec<Sensor>> {
    sqlx::query_as!(
        Sensor,
        r#"
        SELECT external_id, description
        FROM user_sensors
        "#,
    )
    .fetch_all(conn)
    .await
}
