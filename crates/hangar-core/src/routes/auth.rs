use crate::HangarState;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub(super) struct RegisterRequest {
	username: String,
	email: String,
	nickname: Option<String>,
	// Note hashing is performed by us, not the client
	password: String,
}

#[derive(Serialize)]
pub(super) enum RegisterErrorResponse {

}

#[cfg_attr(feature = "_debug_handler", axum::debug_handler)]
pub(super) async fn register(
    State(state): State<Arc<HangarState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<(), Json<RegisterErrorResponse>> {
	let does_user_exist = sqlx::query("
		SELECT
	")
	Ok(())
}
