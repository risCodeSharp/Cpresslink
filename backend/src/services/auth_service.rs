use crate::db::respository::google_oauth::GoogleAuthRepository;
use crate::models::auth::{CreateGoogleOauthUser, GoogleOauthUserLoginResponse};
use crate::models::events::{NotificationEvent, WorkerEvents};
use crate::utils;
use crate::utils::env_registry::{EnvKey, EnvRegistry};
use crate::{
    config::state::AppState,
    db::respository::user::UserRepository,
    error::AppError,
    models::{
        auth::{LoginRequest, RegisterRequest, UserResponse},
        events::UserCreatedEvent,
    },
    utils::jwt,
};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

const REDIRECT_URI: &str = "http://localhost:3080/api/auth/google/callback";

pub async fn register(
    state: &AppState,
    req: RegisterRequest,
) -> Result<Option<UserResponse>, AppError> {
    let hashed_password = hash(req.password, DEFAULT_COST).unwrap();
    let user =
        UserRepository::create(&state.pool, &req.username, &req.email, &hashed_password).await?;

    Ok(match user {
        Some(v) => {
            let _ = state
                .redis_publisher
                .publish(
                    WorkerEvents::Notification,
                    &NotificationEvent::UserCreated(UserCreatedEvent {
                        user_id: v.id,
                        email: v.email.clone(),
                        name: v.username.clone(),
                    }),
                )
                .await;
            Some(UserResponse {
                id: v.id,
                username: v.username,
                email: v.email,
                created_at: v.created_at,
            })
        }
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
    let token = jwt::generate_token(user.id, &jwt_secret)
        .map_err(|e| AppError::FailedToCreateJwtToken(e.to_string()))?;

    Ok(token)
}

pub fn google_auth_url(client_id: &str) -> String {
    format!(
        "https://accounts.google.com/o/oauth2/v2/auth\
        ?client_id={}\
        &redirect_uri={}\
        &response_type=code\
        &scope=openid%20email%20profile",
        client_id, REDIRECT_URI
    )
}

#[derive(Debug, Deserialize)]
struct GoogleClaims {
    sub: String,
    email: Option<String>,
    name: Option<String>,
}

pub async fn get_google_oauth_token(
    env_registry: &EnvRegistry,
    authorization_code: String,
) -> Result<String, AppError> {
    let client = reqwest::Client::new();
    let params: [(&str, String); 5] = [
        ("code", authorization_code),
        ("client_id", env_registry.get(EnvKey::ClientId)),
        ("client_secret", env_registry.get(EnvKey::ClientSecret)),
        ("redirect_uri", REDIRECT_URI.into()),
        ("grant_type", "authorization_code".to_string()),
    ];

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::Unknown(format!("Failed fetching google oauth token {}", e)))?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    if !status.is_success() {
        return Err(AppError::Unknown(format!("API Error: {body}")));
    }

    let token_res: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| AppError::Unknown(e.to_string()))?;

    // let access_token = token_res["access_token"]
    //     .as_str()
    //     .ok_or(AppError::Unknown("Missing access_token".into()))?
    //     .to_string();
    let id_token = token_res["id_token"]
        .as_str()
        .ok_or(AppError::Unknown("Missing id_token".into()))?
        .to_string();
    Ok(id_token)
}

fn extract_google_claims(id_token: &str) -> Result<GoogleClaims, AppError> {
    let token = jsonwebtoken::dangerous::insecure_decode::<GoogleClaims>(id_token)
        .map_err(|_| AppError::InvalidToken)?;

    Ok(token.claims)
}

pub async fn google_oauth_login(
    state: &AppState,
    code: String,
) -> Result<Option<GoogleOauthUserLoginResponse>, AppError> {
    let id_token = get_google_oauth_token(&state.env_keys, code).await?;
    let google_claims = extract_google_claims(&id_token)?;

    let new_user: CreateGoogleOauthUser = CreateGoogleOauthUser {
        username: google_claims.name.unwrap(),
        email: google_claims.email.unwrap(),
        oauth_id: google_claims.sub,
    };
    let user_res = GoogleAuthRepository::find_or_create(&state.pool, new_user).await?;

    let jwt_token = jwt::generate_token(
        user_res.id,
        &state.env_keys.get(utils::env_registry::EnvKey::JwtSecret),
    )
    .map_err(|e| AppError::FailedToCreateJwtToken(e.to_string()))?;
    return Ok(Some(GoogleOauthUserLoginResponse::new(user_res, jwt_token)));
}

#[derive(Serialize, Deserialize)]
pub struct GoogleAuthUserInfo {
    name: String,
    email: String,
}

// pub async fn fetch_user_info(access_token: &str) -> Result<GoogleAuthUserInfo, String> {
//     let user = Client::new()
//         .get("https://openidconnect.googleapis.com/v1/userinfo")
//         .bearer_auth(access_token)
//         .send()
//         .await
//         .map_err(|e| e.to_string())?
//         .json::<GoogleAuthUserInfo>()
//         .await
//         .map_err(|e| e.to_string())?;

//     Ok(user)
// }
