use anyhow::Context;
use nostr::{Event, Metadata, ToBech32, Url};
use nostr_relay_pool::relay::ReqExitPolicy;
use nostr_sdk::{FromBech32, SubscribeAutoCloseOptions};
use sqlx::PgPool;

pub async fn send_job_request(seller_pubkey: String) -> nostr::Result<nostr::EventId> {
    let keys = nostr::Keys::generate();

    // Or use your already existing (from hex or bech32)
    // let keys = Keys::parse("hex-or-bech32-secret-key")?;

    // Show bech32 public key
    let bech32_pubkey: String = keys.public_key().to_bech32()?;
    println!("Bech32 PubKey: {}", bech32_pubkey);

    let client = nostr_sdk::Client::builder().signer(keys.clone()).build();
    client.add_relay("wss://relay.damus.io").await?;
    client.add_read_relay("wss://relay.nostr.info").await?;

    client.connect().await;

    let metadata = Metadata::new()
        .name("nic")
        .display_name("nic")
        .about("Description")
        .picture(Url::parse("https://example.com/avatar.png")?)
        .banner(Url::parse("https://example.com/banner.png")?)
        .nip05("nicolasauler@bipa.app")
        .lud16("nicolasauler@bipa.app");
    // .custom_field("custom_field", "my value");
    client.set_metadata(&metadata).await?;

    let start_tag = nostr::event::Tag::from_standardized(nostr::event::TagStandard::Starts(
        nostr_sdk::Timestamp::now(),
    ));
    let end_tag = nostr::event::Tag::from_standardized(nostr::event::TagStandard::Ends(
        nostr_sdk::Timestamp::now(),
    ));
    let seller_tag = nostr::event::Tag::from_standardized(nostr::event::TagStandard::public_key(
        nostr::PublicKey::from_bech32(&seller_pubkey)?,
    ));

    let tags = nostr::event::Tags::new(vec![start_tag, end_tag, seller_tag]);
    let builder = nostr::EventBuilder::job_request(nostr::event::Kind::from_u16(5000))?.tags(tags);
    let output = client.send_event_builder(builder).await?;

    Ok(*output.id())
}

// /// listen for job events to send results for
// async fn listen_for_jobs(db_pool: PgPool) -> anyhow::Result<Vec<Event>> {
//     let keys = nostr::Keys::generate();
//     let client = nostr_sdk::Client::builder().signer(keys.clone()).build();
//     client.add_relay("wss://relay.damus.io").await?;
//     client.add_read_relay("wss://relay.nostr.info").await?;
//     client.connect().await;
//
//     let pubkeys: Vec<_> = crate::queries::user::list_pubkeys(&db_pool)
//         .await?
//         .into_iter()
//         .map(|p| nostr::PublicKey::from_bech32(p.pubkey.as_str()))
//         .collect::<Result<_, _>>()?;
//
//     let subscription = nostr::Filter::new()
//         .pubkeys(pubkeys)
//         .since(nostr::Timestamp::now());
//     let opts = SubscribeAutoCloseOptions::default().exit_policy(ReqExitPolicy::ExitOnEOSE);
//     let output = client.subscribe(subscription, Some(opts)).await?;
//     println!("Subscription ID: {} [auto-closing]", output.val);
// }

pub async fn listen_for_job_requests(zbd_apikey: String, db_pool: PgPool) -> anyhow::Result<()> {
    let keys = nostr::Keys::generate();
    let client = nostr_sdk::Client::builder().signer(keys.clone()).build();
    client.add_relay("wss://relay.damus.io").await?;
    client.add_read_relay("wss://relay.nostr.info").await?;
    client.connect().await;

    let pubkeys: Vec<_> = crate::queries::user::list_pubkeys(&db_pool)
        .await?
        .into_iter()
        .map(|p| nostr::PublicKey::from_bech32(p.pubkey.as_str()))
        .collect::<Result<_, _>>()?;
    let filter = nostr::Filter::new()
        //.pubkeys(pubkeys)
        .kind(nostr::Kind::from_u16(5000));
    //.since(nostr::Timestamp::now());
    let opts = SubscribeAutoCloseOptions::default().exit_policy(ReqExitPolicy::ExitOnEOSE);
    client
        .subscribe(filter, Some(opts))
        .await
        .context("failed to subscribe")?;

    let mut notifications = client.notifications();
    while let Ok(notification) = notifications.recv().await {
        if let nostr_sdk::RelayPoolNotification::Event { event, .. } = notification {
            process_job_request(zbd_apikey.clone(), event).await?;
        }
    }

    Ok(())
}

async fn process_job_request(zbd_apikey: String, event: Box<Event>) -> anyhow::Result<()> {
    let event_id = send_job_result(zbd_apikey, *event).await?;
    tracing::info!(?event_id, "job result sent");

    Ok(())
}

async fn send_job_result(zbd_apikey: String, job_request: Event) -> anyhow::Result<nostr::EventId> {
    let keys = nostr::Keys::generate();
    let client = nostr_sdk::Client::builder().signer(keys.clone()).build();
    client.add_relay("wss://relay.damus.io").await?;
    client.add_read_relay("wss://relay.nostr.info").await?;
    client.connect().await;

    let amount = 1000;
    let invoice = crate::features::zbd::create_charge(zbd_apikey, amount)
        .await
        .context("failed to create charge in zbd")?;

    let job = nostr::EventBuilder::job_result(
        job_request,
        "dados dos sensores",
        amount * 1000,
        Some(invoice.uri),
    )?;
    let output = client
        .send_event_builder(job)
        .await
        .context("failed to send event")?;

    Ok(*output.id())
}
