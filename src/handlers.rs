use axum::{
    response::Response,
    http::{StatusCode, header},
    extract::Json
};
use serde_json::json;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{Header, EncodingKey, encode};
use chrono::{Duration, Utc};

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

pub async fn get_token_handler(Json(user): Json<User>) -> Response<String> {

    match get_jwt(user) {
        Ok(token) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!({ "token": token }).to_string())
            .unwrap(),
        Err(err) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(json!({ "error": format!("{}", err) }).to_string())
            .unwrap(),
    }

}

fn get_jwt(user: User) -> Result<String, jsonwebtoken::errors::Error> {
    println!("user: {:?}", user);
    return generate_jwt(&user.email);
}

fn generate_jwt(user_email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    println!("user: {:?}", user_email);
    let secret = "mijn-super-geheime-sleutel";

    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_email.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()));

    return token;

}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize, Debug)]
pub struct User {
    email: String,
    _password: String,
}