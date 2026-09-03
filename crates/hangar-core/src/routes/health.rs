#[cfg_attr(feature = "_debug_handler", axum::debug_handler)]
pub(crate) async fn get() -> &'static str {
	"ok"
}
