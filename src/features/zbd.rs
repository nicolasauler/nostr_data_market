use zebedee_rust::{ZebedeeClient, charges::Charge, errors::ZebedeeError};

pub async fn create_charge(env: &crate::Env, sats_amount: u32) -> Result<(), ZebedeeError> {
    let apikey = env.zbd_apikey.clone();
    let zebedee_client = ZebedeeClient::new(apikey);

    let charge = Charge {
        amount: sats_amount.to_string(),
        ..Default::default()
    };

    let charges_res = zebedee_client.create_charge(&charge).await?;
    println!("{:?}", charges_res);

    Ok(())
}
