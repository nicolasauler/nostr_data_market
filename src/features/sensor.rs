use crate::queries::sensor::Sensor;
use sqlx::PgPool;

pub async fn create(
    db: PgPool,
    pubkey: &str,
    sensor_id: &str,
    description: &str,
) -> sqlx::Result<()> {
    crate::queries::sensor::create(
        &db,
        pubkey,
        sensor_id,
        description,
        time::OffsetDateTime::now_utc(),
    )
    .await
    .map(|c| {
        if c.rows_affected() > 1 {
            tracing::error!("i really need a macro that cancels the transaction");
        }
    })
}

pub async fn list(db: PgPool) -> sqlx::Result<Vec<Sensor>> {
    crate::queries::sensor::list(&db).await
}
