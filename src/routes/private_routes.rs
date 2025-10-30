use axum::{
    Router,
    routing::{get},
};
pub fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "hi private page" }))
        .route("/456", get(|| async { "private page 456" }))
}
