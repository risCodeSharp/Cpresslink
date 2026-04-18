use crate::{
    db::respository::user::UserRepository,
    error::AppError,
    models::auth::{LoginRequest, RegisterRequest, UserResponse},
    utils::jwt,
};
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::PgPool;

pub async fn register(
    pool: &PgPool,
    req: RegisterRequest,
) -> Result<Option<UserResponse>, AppError> {
    let hashed_password = hash(req.password, DEFAULT_COST).unwrap();
    let user = UserRepository::create(&pool, &req.username, &req.email, &hashed_password).await?;

    Ok(match user {
        Some(v) => Some(UserResponse {
            id: v.id,
            username: v.username,
            email: v.email,
            created_at: v.created_at,
        }),
        None => None,
    })
}

pub async fn login(pool: &PgPool, jwt_secret: &str, req: LoginRequest) -> Result<String, AppError> {
    let user = UserRepository::find_by_email(&pool, &req.email)
        .await?
        .ok_or(AppError::NoUserFound)?;

    let is_valid =
        verify(&req.password, &user.password_hash).map_err(|_| AppError::InvalidCredentials)?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }
    let token = jwt::generate_token(user.id, &jwt_secret).map_err(|e| {
        AppError::Unknown(format!(
            "[JWT Error]: Failed to create token.\n[Details]: {} ",
            e.to_string()
        ))
    })?;

    Ok(token)
}
