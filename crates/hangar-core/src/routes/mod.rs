mod health;

use crate::HangarState;
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn router() -> Router<Arc<HangarState>> {
    Router::new().route("/health", get(health::get))
}
