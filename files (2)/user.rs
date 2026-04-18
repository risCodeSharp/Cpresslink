use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult, ErrorResponse},
    middleware::auth::{AuthUser, Claims},
    models::user::{
        CreateUserRequest, LoginRequest, LoginResponse, UpdateUserRequest, UserResponse,
        UsersListResponse,
    },
    AppState,
};

// ─── POST /api/v1/auth/register ───────────────────────────────────────────────

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Auth",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Bad request / validation error", body = ErrorResponse),
        (status = 409, description = "Email or username already taken", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> AppResult<(StatusCode, Json<UserResponse>)> {
    // Basic validation
    if body.email.is_empty() || body.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Email required and password must be at least 8 characters".into(),
        ));
    }

    let password_hash = hash(&body.password, DEFAULT_COST).map_err(AppError::Bcrypt)?;

    let user = sqlx::query_as!(
        crate::models::user::User,
        r#"
        INSERT INTO users (id, email, username, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
        Uuid::new_v4(),
        body.email,
        body.username,
        password_hash,
        Utc::now(),
        Utc::now(),
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Email or username already exists".into())
        }
        other => AppError::Database(other),
    })?;

    Ok((StatusCode::CREATED, Json(user.into())))
}

// ─── POST /api/v1/auth/login ──────────────────────────────────────────────────

/// Login and receive a JWT token
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let user = sqlx::query_as!(
        crate::models::user::User,
        "SELECT * FROM users WHERE email = $1",
        body.email
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

    let valid = verify(&body.password, &user.password_hash).map_err(AppError::Bcrypt)?;
    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".into()));
    }

    let exp = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(AppError::Jwt)?;

    Ok(Json(LoginResponse {
        token,
        user: user.into(),
    }))
}

// ─── GET /api/v1/users ────────────────────────────────────────────────────────

/// List all users (requires auth)
#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "Users",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of users", body = UsersListResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
    Extension(_auth_user): Extension<AuthUser>,
) -> AppResult<Json<UsersListResponse>> {
    let users = sqlx::query_as!(
        crate::models::user::User,
        "SELECT * FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await?;

    let total = users.len();
    let users: Vec<UserResponse> = users.into_iter().map(Into::into).collect();

    Ok(Json(UsersListResponse { users, total }))
}

// ─── GET /api/v1/users/:id ────────────────────────────────────────────────────

/// Get a single user by ID (requires auth)
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "Users",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<UserResponse>> {
    let user = sqlx::query_as!(
        crate::models::user::User,
        "SELECT * FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

    Ok(Json(user.into()))
}

// ─── PUT /api/v1/users/:id ────────────────────────────────────────────────────

/// Update a user (requires auth, must be own account)
#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "Users",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden – not your account", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 409, description = "Email or username already taken", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn update_user(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    // Only allow users to update their own account
    if auth_user.0.sub != id {
        return Err(AppError::Forbidden("You can only update your own account".into()));
    }

    let user = sqlx::query_as!(
        crate::models::user::User,
        r#"
        UPDATE users
        SET
            email      = COALESCE($1, email),
            username   = COALESCE($2, username),
            updated_at = $3
        WHERE id = $4
        RETURNING *
        "#,
        body.email,
        body.username,
        Utc::now(),
        id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Email or username already exists".into())
        }
        other => AppError::Database(other),
    })?
    .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

    Ok(Json(user.into()))
}

// ─── DELETE /api/v1/users/:id ─────────────────────────────────────────────────

/// Delete a user (requires auth, must be own account)
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "Users",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden – not your account", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    if auth_user.0.sub != id {
        return Err(AppError::Forbidden("You can only delete your own account".into()));
    }

    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("User {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}
