use crate::AppState;
use axum::{Router, extract::State, http::StatusCode, routing::get};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/simulate", get(simulate))
        .route("/api/send-job", get(send_job))
        .route("/api/process-job", get(process_jobs))
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
