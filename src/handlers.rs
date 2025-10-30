use axum::{
    response::Response,
    http::{StatusCode, header},
};
use serde_json::json;

pub async fn public_view_handler() -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(json!({
            "success": true,
            "message": "Hello World!"
        }).to_string())
        .unwrap()

}

pub async fn get_token_handler() -> Response<String> {
    todo!()
}