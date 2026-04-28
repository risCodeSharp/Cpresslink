use std::net::SocketAddr;

use axum::{
    Json, Router, extract::{ConnectInfo, Path, Query, State}, http::{self, HeaderMap, StatusCode}, middleware::Next, response::{self, IntoResponse, Redirect}, routing
};
use serde::Deserialize;
use tracing::{debug, info};

use crate::{
    config::state::AppState, services::redirect_service::RedirectService, utils::{response::ApiResponse}
};

#[derive(Deserialize)]
struct ReferrerQuery {
    rf: Option<String>,
}

async fn redirect_time_measurer_middleware(
    req: http::Request<axum::body::Body>,
    next: Next,
) -> response::Response {
    let url = req.uri().path().to_string();
    let start = std::time::Instant::now();
    let response = next.run(req).await;

    let elasped = start.elapsed().as_millis().to_string();
    info!("code={url} latency={elasped} ms");
    response
}


async fn redirect_by_slug(
    State(state): State<AppState>,
    Path((username, slug)): Path<(String, String)>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(q): Query<ReferrerQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let user_agent = headers.get(http::header::USER_AGENT).and_then(|v| v.to_str().ok()).unwrap_or("Unknown");
    let refer = q.rf;
    info!("referrer={:?} ip={}, ua={}", refer, addr.ip(), user_agent);
    match RedirectService::get_by_slug(state, &username, &slug, addr.ip().to_string(), user_agent.to_string(), refer).await {
        Ok(url) => Redirect::to(&url.destination).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(&e.to_string(), StatusCode::NOT_FOUND)),
        ).into_response(),
    }
}
async fn redirect_by_code(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(q): Query<ReferrerQuery>,
    headers: HeaderMap,
    
) -> impl IntoResponse {
    let user_agent = headers.get(http::header::USER_AGENT).and_then(|v| v.to_str().ok()).unwrap_or("Unknown");
    let refer = q.rf;
    println!("referrer={:?} ip={}, ua={}", refer, addr.ip(), user_agent);
    match RedirectService::get_by_code(state, short_code, addr.ip().to_string(), user_agent.to_string(), refer).await {
        Ok(url) => Redirect::to(&url.destination).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(&e.to_string(), StatusCode::NOT_FOUND)),
        ).into_response(),
    }
}


pub fn router() -> Router<AppState> {
    Router::new()
    .route("/u/{username}/{slug}", routing::get(redirect_by_slug))
    .route("/r/{short_code}", routing::get(redirect_by_code))
    .layer(axum::middleware::from_fn(redirect_time_measurer_middleware))
}
