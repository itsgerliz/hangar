mod auth;
mod health;

use crate::HangarState;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn router() -> Router<Arc<HangarState>> {
    Router::new()
        .route("/health", get(health::health))
        .route("/api/auth/register", post(auth::register))
}
