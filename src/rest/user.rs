use crate::AppState;
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::Deserialize;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/api/signup", post(signup))
}

#[derive(Deserialize)]
pub struct SignUpInput {
    pub username: String,
    pub pubkey: String,
}

async fn signup(
    State(shared_state): State<Arc<AppState>>,
    Json(input): Json<SignUpInput>,
) -> StatusCode {
    let pubkey = input.pubkey.clone();

    match crate::features::user::create(shared_state.pool.clone(), input).await {
        Ok(()) => StatusCode::OK,
        Err(e) => {
            tracing::error!(pubkey, ?e, "error creating user");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
