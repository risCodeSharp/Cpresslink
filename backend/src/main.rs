use std::net::SocketAddr;

use axum::{ Router, extract::ConnectInfo, http::{HeaderMap, Method},  routing::get};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::config::state::AppState;

pub mod handlers;
pub mod models;
pub mod config;
pub mod services;
pub mod queue;
pub mod workers;
pub mod db;
pub mod error;
pub mod middleware;
pub mod utils;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

fn get_client_ip(headers: &HeaderMap, connect_info: Option<SocketAddr>) -> Option<String> {
    if let Some(forwared) = headers.get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwared.to_str() {
            return forwarded_str.split(",").next().map(|s| s.trim().to_string());
        }
    }

    connect_info.map(|addr| addr.ip().to_string())
}

async fn your_ip(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> String {
    let ip = get_client_ip(&headers, Some(addr));
    println!("Client IP: {:?}", ip);
    ip.unwrap()
}


pub async fn run() {
    // loading the .env to project
    dotenvy::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(Any);
          
    let state = AppState::new().await;

    let router: Router = Router::new()
        .merge(handlers::auth::router())
        .merge(handlers::link::router(state.clone()))
        .merge(handlers::redirect::router())
        .route("/test", get(your_ip) )
        .with_state(state)
        .layer(cors)
        .merge(
            SwaggerUi::new("/docs")
            .url("/docs/openapi.json", ApiDoc::openapi()),
        );

    let port = 3080;
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Port is already been used!");

    println!("Listening on :{addr}");

    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Error serving application");
}


#[tokio::main]
async fn main() {
    run().await;
}
