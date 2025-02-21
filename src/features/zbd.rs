use anyhow::Context;
use zebedee_rust::{
    ZebedeeClient,
    charges::{Charge, InvoiceData},
};

pub async fn create_charge(zbd_apikey: String, sats_amount: u64) -> anyhow::Result<InvoiceData> {
    let zebedee_client = ZebedeeClient::new(zbd_apikey);

    let charge = Charge {
        amount: sats_amount.to_string(),
        ..Default::default()
    };

    let charges_res = zebedee_client.create_charge(&charge).await?;
    tracing::info!(?charges_res, "zbd bolt11");

    let invoice = charges_res
        .data
        .context("missing bolt11 data")?
        .invoice
        .context("missing invoice data")?;

    Ok(invoice)
}
