use crate::AppState;
use axum::{Router, extract::State, http::StatusCode, routing::get};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/api/simulate", get(simulate))
}

async fn simulate(State(shared_state): State<Arc<AppState>>) -> StatusCode {
    match crate::features::zbd::create_charge(&shared_state.env, 1).await {
        Ok(()) => StatusCode::OK,
        Err(e) => {
            tracing::error!(?e, "error creating user");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
