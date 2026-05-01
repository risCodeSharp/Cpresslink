use crate::{
    config::state::AppState,
    utils::{jwt, response::ApiResponse, env_registry::EnvKey},
};
use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::errors::ErrorKind;
use tracing::info;

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    let state = req
    .extensions()
    .get::<AppState>()
    .cloned()
    .ok_or(StatusCode::INTERNAL_SERVER_ERROR.into_response())?;
// Extracting token form the header
info!("Has state: {}", req.extensions().get::<AppState>().is_some());
let token = req
.headers()
.get("Authorization")
.and_then(|h| h.to_str().ok())
.and_then(|h| h.strip_prefix("Bearer "))
.ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<()>::error(
                    "token not found",
                    StatusCode::UNAUTHORIZED,
                )),
            )
                .into_response()
        })?;

    // Serialize the struct from the token
    let claims = jwt::verify_token(token, &state.env_keys.get(EnvKey::JwtSecret )).map_err(|e| {
        let res = match e.kind() {
            ErrorKind::ExpiredSignature => {
                ApiResponse::<()>::error("Token expired", StatusCode::UNAUTHORIZED)
            }
            ErrorKind::InvalidToken => {
                ApiResponse::<()>::error("Invalid token", StatusCode::UNAUTHORIZED)
            }
            _ => ApiResponse::<()>::error("Authenication failed", StatusCode::UNAUTHORIZED),
        };

        (StatusCode::UNAUTHORIZED, Json(res)).into_response()
    })?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
