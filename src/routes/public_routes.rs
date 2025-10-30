use axum::{
    Router,
    routing::{get},
};
pub fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "hi public page" }))
        .route("/123", get(|| async { "public page 123" }))
}
