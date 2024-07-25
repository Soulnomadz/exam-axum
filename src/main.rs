use axum::{
    routing::get,
    Router,
};

use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // let routes_hello = Router::new().route(
    //     "/hello",
    //     get(|| async { Html("Hello <strong>World!!!</strong>") })
    // );

    let app = Router::new()
        .route("/hello", get(hello));

    let addr = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    info!("=>> Listening on {:?}", addr);
    
    axum::serve(addr, app)
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello world!"
}

