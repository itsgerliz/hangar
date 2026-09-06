use crate::HangarState;
use axum::{Json, extract::State};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub(super) struct HealthResponse {
    status: &'static str,
    uptime: String,
}

#[cfg_attr(feature = "_debug_handler", axum::debug_handler)]
pub(super) async fn get(State(state): State<Arc<HangarState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        uptime: state.uptime(),
    })
}
