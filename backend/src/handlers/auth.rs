use crate::config::state::AppState;
use crate::{
    models::auth::{LoginRequest, RegisterRequest},
    services::auth_service,
    utils::{env_registry::EnvKey, response::ApiResponse},
};
use serde::Deserialize;
use axum::extract::Query;
use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing,
};

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    match auth_service::register(&state, payload).await {
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
    match auth_service::login(&state.pool, &state.env_keys.get(EnvKey::JwtSecret), payload).await {
        Ok(token) => (
            StatusCode::OK,
            Json(ApiResponse::success(token, StatusCode::OK)),
        ),
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(e.to_string(), StatusCode::UNAUTHORIZED)),
        ),
    }
}


pub async fn google_login(State(state): State<AppState>) -> Redirect {
    let client_id = &state.env_keys.get(EnvKey::ClientId);
    let url = auth_service::google_auth_url(client_id);
    Redirect::to(&url)
}


#[derive( Deserialize)]
struct GoogleAuthQuery {
    code: String,
}

async fn google_callback(
    State(state): State<AppState>,
    Query(query): Query<GoogleAuthQuery>,
) -> impl IntoResponse {
      match auth_service::google_oauth_login(&state, query.code ).await {
        Ok(token) => (
            StatusCode::OK,
            Json(ApiResponse::success(token, StatusCode::OK)),
        ),
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(e.to_string(), StatusCode::UNAUTHORIZED)),
        ),
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", routing::post(register))
        .route("/login", routing::post(login))
        .route("/google", routing::get(google_login))
        .route("/google/callback", routing::get(google_callback))
}


/*

async fn google_callback(
    State(state): State<AppState>,
    Query(query): Query<GoogleAuthQuery>,
) -> impl IntoResponse {

    let access_token = match auth_service::request_access_token(&state.env_keys, query.code).await {
        Ok(token) => token,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(ApiResponse::error("Token exchange failed", StatusCode::UNAUTHORIZED)));
        }
    };

    let user_info = match auth_service::fetch_user_info(&access_token).await {
        Ok(info) => info,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(ApiResponse::error("Failed to fetch user info", StatusCode::UNAUTHORIZED)));
        }
    };

    // 👇 IMPORTANT PART STARTS HERE

    // Extract fields
    let google_id = user_info.sub.clone();
    let email = user_info.email.clone();
    let name = user_info.name.clone();
    let avatar = user_info.picture.clone();

    // Check if user exists
    let user = match user_repository::find_by_google_id(&state.db, &google_id).await {
        Ok(Some(user)) => user,

        Ok(None) => {
            // Create new user
            match user_repository::create_user(
                &state.db,
                google_id,
                email,
                name,
                avatar,
            ).await {
                Ok(new_user) => new_user,
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Failed to create user", StatusCode::INTERNAL_SERVER_ERROR)));
                }
            }
        }

        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("DB error", StatusCode::INTERNAL_SERVER_ERROR)));
        }
    };

    // Create JWT (or session)
    let token = match auth_service::generate_jwt(user.id) {
        Ok(token) => token,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error("Token creation failed", StatusCode::INTERNAL_SERVER_ERROR)));
        }
    };

    // Return token + user
    (
        StatusCode::OK,
        Json(ApiResponse::success(
            serde_json::json!({
                "user": user,
                "token": token
            }),
            StatusCode::OK,
        )),
    )
}


*/