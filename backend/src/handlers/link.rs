use axum::{
    Extension, Router,
    extract::{Json, Path, State},
    http::StatusCode,
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing,
};

use crate::{
    config::state::AppState,
    middleware::auth::auth_middleware,
    models::links::CreateLinkRequest,
    services::link_service::LinkService,
    utils::{jwt::JwtClaims, response::ApiResponse},
};

pub async fn create_link(
    State(state): State<AppState>,
    Extension(jwt_claims): Extension<JwtClaims>,
    Json(payload): Json<CreateLinkRequest>,
) -> impl IntoResponse {
    println!("Check!");
    match LinkService::create(&state, payload, jwt_claims.sub).await {
        Ok(link) => (
            StatusCode::CREATED,
            Json(ApiResponse::success(link, StatusCode::CREATED)),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(e.to_string(), StatusCode::BAD_REQUEST)),
        ),
    }
}

pub async fn get_link(
    State(state): State<AppState>,
    Extension(jwt_claims): Extension<JwtClaims>,
    Path(link_id): Path<i64>,
) -> impl IntoResponse {
    match LinkService::get_link_by_id(&state.pool, jwt_claims.sub, link_id).await {
        Ok(link) => (
            StatusCode::OK,
            Json(ApiResponse::success(link, StatusCode::OK)),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(e.to_string(), StatusCode::BAD_REQUEST)),
        ),
    }
}

pub async fn list_links(
    State(state): State<AppState>,
    Extension(jwt_claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match LinkService::list_links(&state.pool, jwt_claims.sub).await {
        Ok(links) => (
            StatusCode::OK,
            Json(ApiResponse::success(links, StatusCode::OK)),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(e.to_string(), StatusCode::BAD_REQUEST)),
        ),
    }   
}

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/links", routing::get(list_links))
        .route("/links/{slug}", routing::get(get_link))
        .route("/links", routing::post(create_link))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
}
