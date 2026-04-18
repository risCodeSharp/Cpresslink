use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, AppState};

// ─── JWT Claims ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,   // user id
    pub email: String,
    pub exp: usize,  // expiry (unix timestamp)
    pub iat: usize,  // issued at
}

// ─── Extractor: authenticated user pulled from token ─────────────────────────

#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

// ─── Middleware function ──────────────────────────────────────────────────────

pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing or malformed Authorization header".into()))?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(AppError::Jwt)?
    .claims;

    req.extensions_mut().insert(AuthUser(claims));
    Ok(next.run(req).await)
}
