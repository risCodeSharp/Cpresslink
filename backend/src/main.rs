use axum::{
    Error, Router,
    extract::{Json, State},
    http::Method,
    routing,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let router: Router = Router::new()
        .route("/health", routing::get(  async || "Hello Links!"))
        .layer(cors);

    let port = 3080;
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Port is already been used!");

    println!("Listening on :{addr}");

    axum::serve(listener, router)
        .await
        .expect("Error serving application");
}
