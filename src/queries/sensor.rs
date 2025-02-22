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
    pub user_pubkey: String,
}

pub async fn list(conn: impl PgExecutor<'_>) -> sqlx::Result<Vec<Sensor>> {
    sqlx::query_as!(
        Sensor,
        r#"
        SELECT external_id, description, user_pubkey
        FROM user_sensors
        "#,
    )
    .fetch_all(conn)
    .await
}

pub struct Data {
    pub payload: serde_json::Value,
}

pub async fn get_all_data(conn: impl PgExecutor<'_>) -> sqlx::Result<Vec<Data>> {
    sqlx::query_as!(
        Data,
        r#"
        SELECT payload
        FROM mqtt_raw
        "#,
    )
    .fetch_all(conn)
    .await
}

pub async fn pubkey_for_sensor(
    conn: impl PgExecutor<'_>,
    external_id: &str,
) -> sqlx::Result<String> {
    sqlx::query!(
        r#"
        SELECT user_pubkey
        FROM user_sensors
        WHERE external_id = $1
        "#,
        external_id,
    )
    .fetch_one(conn)
    .await
    .map(|row| row.user_pubkey)
}
