use crate::{
    config::state::AppState,
    utils::{jwt, response::ApiResponse, env_registry::EnvKey},
};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use jsonwebtoken::errors::ErrorKind;
use tracing::info;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    // -----------------------------
    // Extract Authorization header
    // -----------------------------
    let token = match req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    {
        Some(token) => token,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<()>::error(
                    "token not found",
                    StatusCode::UNAUTHORIZED,
                )),
            )
                .into_response();
        }
    };

    // -----------------------------
    // Verify JWT
    // -----------------------------
    let claims = match jwt::verify_token(
        token,
        &state.env_keys.get(EnvKey::JwtSecret),
    ) {
        Ok(claims) => claims,
        Err(e) => {
            let res = match e.kind() {
                ErrorKind::ExpiredSignature => {
                    ApiResponse::<()>::error("Token expired", StatusCode::UNAUTHORIZED)
                }
                ErrorKind::InvalidToken => {
                    ApiResponse::<()>::error("Invalid token", StatusCode::UNAUTHORIZED)
                }
                _ => ApiResponse::<()>::error(
                    "Authentication failed",
                    StatusCode::UNAUTHORIZED,
                ),
            };

            return (StatusCode::UNAUTHORIZED, Json(res)).into_response();
        }
    };

    // -----------------------------
    // Attach claims to request
    // -----------------------------
    req.extensions_mut().insert(claims);

    // continue request
    next.run(req).await
}