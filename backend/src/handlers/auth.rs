use crate::config::state::AppState;
use crate::{
    models::auth::{LoginRequest, RegisterRequest},
    services::auth_service,
    utils::response::ApiResponse,
};
use axum::Router;
use axum::{
    extract::{Json, State},
    http::{StatusCode},
    response::IntoResponse,
    routing,
};

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    match auth_service::register(&state.pool, payload).await {
        Ok(Some(user)) => (
            StatusCode::OK,
            Json(ApiResponse::success(user, StatusCode::OK)),
        ),
        Ok(None) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(
                "Registration failed",
                StatusCode::BAD_REQUEST,
            )),
        ),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                e.to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        ),
    }
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match auth_service::login(&state.pool, &state.jwt_secret, payload).await {
        Ok(token) => (StatusCode::OK, Json(ApiResponse::success(token, StatusCode::OK))),
        Err(e) => (StatusCode::UNAUTHORIZED, Json(ApiResponse::error(e.to_string(), StatusCode::UNAUTHORIZED))),
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
    .route("/register", routing::post(register))
    .route("/login", routing::post(login))
}
