mod features;
mod queries;
mod rest;

use anyhow::Context;
use axum::Router;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct Env {
    db_url: String,
    instance_id: String,
    otel_endpoint: String,
    rust_log: String,
    zbd_apikey: String,
}

impl Env {
    fn try_build() -> Result<Self, std::env::VarError> {
        let db_url = std::env::var("DATABASE_URL")?;
        let instance_id = std::env::var("INSTANCE_ID")?;
        let otel_endpoint = std::env::var("OTEL_ENDPOINT")?;
        let rust_log = std::env::var("RUST_LOG")?;
        let zbd_apikey = std::env::var("ZBD_APIKEY")?;

        Ok(Self {
            db_url,
            instance_id,
            otel_endpoint,
            rust_log,
            zbd_apikey,
        })
    }
}

/// The application state to be shared in axum.
struct AppState {
    /// The postgres connection pool.
    pool: PgPool,
    /// The environment variables
    env: Env,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::try_build().context("couldn't build envs")?;

    let logger_provider = init_otlp(
        &env.rust_log,
        &env.otel_endpoint,
        "server",
        "0",
        &env.instance_id,
    )
    .context("can't start logging")?;

    let db_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&env.db_url)
        .await
        .context("can't connect to database")?;

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .context("couldn't run migrations")?;

    let shared_state = Arc::new(AppState { pool: db_pool, env });

    tokio::select! {
        rest_result = rest(shared_state.clone()) => {
            rest_result??;
        }
    }

    logger_provider.shutdown()?;
    Ok(())
}

fn rest(app_state: Arc<AppState>) -> tokio::task::JoinHandle<anyhow::Result<()>> {
    let router = Router::new()
        .merge(rest::user::router())
        .merge(rest::shop::router())
        .with_state(app_state);

    tracing::info!("Server started");

    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
            .await
            .context("could not start listener")?;
        let server = axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        );

        tracing::info!("REST ready to go at http://127.0.0.1:8000");
        let outcome = server.await;
        tracing::info!("REST went bye bye.");
        outcome.context("server")
    })
}

fn init_otlp(
    rust_log: &str,
    otel_endpoint: &str,
    name: &'static str,
    version: &'static str,
    instance_id: &str,
) -> Result<opentelemetry_sdk::logs::LoggerProvider, opentelemetry_sdk::logs::LogError> {
    let logger_provider = init_logs(otel_endpoint, name, version, instance_id)?;

    let otel_logger =
        opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&logger_provider);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(rust_log))
        .with(tracing_subscriber::fmt::layer())
        .with(otel_logger)
        .init();

    Ok(logger_provider)
}

fn init_logs(
    otel_endpoint: &str,
    name: &'static str,
    version: &'static str,
    instance_id: &str,
) -> Result<opentelemetry_sdk::logs::LoggerProvider, opentelemetry_sdk::logs::LogError> {
    use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
    use opentelemetry_semantic_conventions::resource::{
        SERVICE_INSTANCE_ID, SERVICE_NAME, SERVICE_VERSION,
    };

    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint(otel_endpoint)
        .with_compression(opentelemetry_otlp::Compression::Zstd)
        .build()?;

    Ok(opentelemetry_sdk::logs::LoggerProvider::builder()
        .with_resource(opentelemetry_sdk::Resource::new([
            opentelemetry::KeyValue::new(SERVICE_NAME, name),
            opentelemetry::KeyValue::new(SERVICE_VERSION, version),
            opentelemetry::KeyValue::new(SERVICE_INSTANCE_ID, instance_id.to_owned()),
        ]))
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .build())
}
