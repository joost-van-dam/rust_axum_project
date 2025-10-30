use axum::{
    routing::{get, post},
    Router,
};

use tokio::net::TcpListener;
// use serde_json::json;

mod handlers;
use handlers::{public_view_handler, get_token_handler};
// use handlers::User;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let routes = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/public-view", get(public_view_handler))
        .route("/get-token", post(get_token_handler))
        .route("/secret-view", get(|| async { "Secret View" }));

    // run our app with hyper, listening globally on port 7000
    let listener = TcpListener::bind("0.0.0.0:7000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();

}