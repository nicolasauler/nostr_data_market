use crate::{AppState, features::sensor::DataBuy};
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use std::sync::Arc;
use time::OffsetDateTime;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/simulate", get(simulate))
        .route("/api/send-job", get(send_job))
        .route("/api/process-job", get(process_jobs))
        .route("/api/buy", get(buy_data))
}

#[derive(Deserialize)]
struct BuyData {
    from: OffsetDateTime,
    to: OffsetDateTime,
    sensor_external_id: String,
}

async fn buy_data(
    State(shared_state): State<Arc<AppState>>,
    Query(data_input): Query<BuyData>,
) -> Result<Json<DataBuy>, StatusCode> {
    match crate::features::sensor::get_all_data(
        shared_state.pool.clone(),
        shared_state.env.zbd_apikey.clone(),
        &data_input.sensor_external_id,
    )
    .await
    {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            tracing::error!(?e, "error sending job");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn send_job() -> StatusCode {
    match crate::features::nostr::send_job_request(
        "npub1yrmhlvasagpzzmxstuu0y7zwvc7mqtp75t3gtmdql7ayqtrzrn4setw7nt".to_owned(),
    )
    .await
    {
        Ok(_id) => StatusCode::OK,
        Err(e) => {
            tracing::error!(?e, "error sending job");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn process_jobs(State(shared_state): State<Arc<AppState>>) -> StatusCode {
    match crate::features::nostr::listen_for_job_requests(
        shared_state.env.zbd_apikey.clone(),
        shared_state.pool.clone(),
    )
    .await
    {
        Ok(()) => StatusCode::OK,
        Err(e) => {
            tracing::error!(?e, "error processing jobs");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn simulate(State(shared_state): State<Arc<AppState>>) -> StatusCode {
    match crate::features::zbd::create_charge(shared_state.env.zbd_apikey.clone(), 1000).await {
        Ok(_invoice) => StatusCode::OK,
        Err(e) => {
            tracing::error!(?e, "error simulating charge");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
