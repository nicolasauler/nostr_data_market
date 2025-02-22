use crate::queries::sensor::Sensor;
use anyhow::Context;
use sqlx::PgPool;
use zebedee_rust::charges::InvoiceData;

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

#[derive(Debug, serde::Serialize)]
pub struct DataBuy {
    pub invoice: InvoiceData,
    pub data: Vec<serde_json::Value>,
}

pub async fn get_all_data(
    db: PgPool,
    zbd_apikey: String,
    sensor_external_id: &str,
) -> anyhow::Result<DataBuy> {
    let payload = crate::queries::sensor::get_all_data(&db)
        .await?
        .into_iter()
        .map(|p| p.payload)
        .collect();
    let pubkey = crate::queries::sensor::pubkey_for_sensor(&db, sensor_external_id).await?;
    tracing::info!("pubkey will be paid: {pubkey}");

    let amount = 1000;
    let invoice = crate::features::zbd::create_charge(zbd_apikey, amount)
        .await
        .context("failed to create charge in zbd")?;

    Ok(DataBuy {
        invoice,
        data: payload,
    })
}
