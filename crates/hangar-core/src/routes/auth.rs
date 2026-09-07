use crate::HangarState;
use axum::{Json, extract::State};
use std::sync::Arc;

pub(super) struct RegisterRequest {
	username: String,
	email: String,
	nickname: Option<String>,
	// Note hashing is performed by us, not the client
	password: String,
}

pub(super) struct RegisterErrorResponse {
	reason: String
}

#[cfg_attr(feature = "_debug_handler", axum::debug_handler)]
pub(super) async fn register(
    State(state): State<Arc<HangarState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<(), RegisterErrorResponse> {

	Ok(())
}
