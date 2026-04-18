use crate::{
    config::state::AppState,
    utils::{jwt, response::ApiResponse},
};
use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::errors::ErrorKind;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    // Extracting token form the header
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
    let claims = jwt::verify_token(token, &state.jwt_secret).map_err(|e| {
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
