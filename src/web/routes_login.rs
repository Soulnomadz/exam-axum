
use crate::{Error, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use axum::{Json, Router, routing::post};

pub fn routes() -> Router {
    Router::new()
       .route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("==>> {:<12} - api_login", "HANDLER");

    if payload.username != "demo" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}