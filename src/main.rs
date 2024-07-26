use axum::{
    extract::{Path, Query}, http::StatusCode, response::{Html, IntoResponse}, routing::{get, get_service, post}, Json, Router
};

use tracing::info;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // let routes_hello = Router::new().route(
    //     "/hello",
    //     get(|| async { Html("Hello <strong>World!!!</strong>") })
    // );

    let app = Router::new()
        .merge(routes_hello())
        .route("/user", post(create_user))
        .fallback_service(routes_static());

    let addr = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    info!("==>> Listening on {:?}", addr);

    axum::serve(addr, app)
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new()
        .nest_service("/", get_service(ServeDir::new("./")))
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    info!("==>> {:<12} - handler_hello - {params:?}", "HANDLER");
    
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    info!("==>> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
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


