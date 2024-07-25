use axum::{
    http::StatusCode, 
    routing::{get, post}, 
    Router, 
    Json,
};

use tracing::info;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // let routes_hello = Router::new().route(
    //     "/hello",
    //     get(|| async { Html("Hello <strong>World!!!</strong>") })
    // );

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/user", post(create_user));

    let addr = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    info!("=>> Listening on {:?}", addr);

    axum::serve(addr, app)
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello world!"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        name: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}


