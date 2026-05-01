use crate::db::respository::github_oauth::GithubAuthRepository;
use crate::db::respository::google_oauth::GoogleAuthRepository;
use crate::models::auth::{AuthResponse, CreateOAuthUser, OAuthUserLoginResponse, User};
use crate::models::events::{NotificationEvent, WorkerEvents};
use crate::queue::RedisPublisher;
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

async fn create_user(
    redis: &RedisPublisher,
    user: Option<User>,
) -> Result<Option<UserResponse>, AppError> {
    Ok(match user {
        Some(v) => {
            let _ = redis
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

pub async fn register(
    state: &AppState,
    req: RegisterRequest,
) -> Result<Option<AuthResponse>, AppError> {
    let hashed_password = hash(req.password, DEFAULT_COST).unwrap();
    let user: Option<User> =
        UserRepository::create(&state.pool, &req.username, &req.email, &hashed_password).await?;
    let user_res = create_user(&state.redis_publisher, user)
        .await?
        .ok_or_else(|| AppError::FailedUserCreatedNotifcation)?;

    let token = jwt::generate_token(user_res.id.clone(), &state.env_keys.get(EnvKey::JwtSecret))
        .map_err(|e| AppError::FailedToCreateJwtToken(e.to_string()))?;
    Ok(Some(AuthResponse {
        user: user_res,
        access_token: token,
    }))
}

pub async fn login(
    pool: &PgPool,
    env_registry: &EnvRegistry,
    req: LoginRequest,
) -> Result<AuthResponse, AppError> {
    let user = UserRepository::find_by_email(&pool, &req.email)
        .await?
        .ok_or(AppError::NoUserFound)?;

    let is_valid =
        verify(&req.password, &user.password_hash).map_err(|_| AppError::InvalidCredentials)?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }
    let token = jwt::generate_token(user.id, &env_registry.get(EnvKey::JwtSecret))
        .map_err(|e| AppError::FailedToCreateJwtToken(e.to_string()))?;

    Ok(AuthResponse {
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        },
        access_token: token,
    })
}

pub fn google_auth_url(env_registry: &EnvRegistry) -> String {
    format!(
        "https://accounts.google.com/o/oauth2/v2/auth\
        ?client_id={}\
        &redirect_uri={}\
        &response_type=code\
        &scope=openid%20email%20profile",
        env_registry.get(EnvKey::GoogleClientId),
        env_registry.get(EnvKey::GoogleRedirectUri)
    )
}
pub fn github_auth_url(env_registry: &EnvRegistry) -> String {
    format!(
        "https://github.com/login/oauth/authorize\
        ?client_id={}\
        &redirect_uri={}\
        &scope=user:email",
        env_registry.get(EnvKey::GithubClientId),
        env_registry.get(EnvKey::GithubRedirectUri),
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
        ("client_id", env_registry.get(EnvKey::GoogleClientId)),
        (
            "client_secret",
            env_registry.get(EnvKey::GoogleClientSecret),
        ),
        ("redirect_uri", env_registry.get(EnvKey::GoogleRedirectUri)),
        ("grant_type", "authorization_code".to_string()),
    ];

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::Unknown(format!("Google token request failed: {}", e)))?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    if !status.is_success() {
        return Err(AppError::Unknown(format!("Github API error: {body}")));
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

pub async fn get_github_oauth_token(
    env_registry: &EnvRegistry,
    authorization_code: String,
) -> Result<String, AppError> {
    let client = reqwest::Client::new();
    let params: [(&str, String); 4] = [
        ("code", authorization_code),
        ("client_id", env_registry.get(EnvKey::GithubClientId)),
        (
            "client_secret",
            env_registry.get(EnvKey::GithubClientSecret),
        ),
        ("redirect_uri", env_registry.get(EnvKey::GithubRedirectUri)),
    ];

    let response = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::Unknown(format!("Github token request failed: {}", e)))?;

    let status = response.status();

    let token_res: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::Unknown(format!("Invalid JSON response: {}", e.to_string())))?;

    if !status.is_success() {
        return Err(AppError::Unknown(format!(
            "Github API error: {:?}",
            token_res
        )));
    }

    let access_token = token_res["access_token"]
        .as_str()
        .ok_or(AppError::Unknown("Missing access_token".into()))?
        .to_string();
    Ok(access_token)
}

fn extract_google_claims(id_token: &str) -> Result<GoogleClaims, AppError> {
    let token = jsonwebtoken::dangerous::insecure_decode::<GoogleClaims>(id_token)
        .map_err(|_| AppError::InvalidToken)?;

    Ok(token.claims)
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub id: u64,
    pub login: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

// fn extract_github_claims(access_token: &str) -> Result<GithubUser, AppError> {
//     println!("{access_token}");
//     let token = jsonwebtoken::dangerous::insecure_decode::<GithubUser>(access_token)
//         .map_err(|e| AppError::Unknown(e.to_string()))?;

//     Ok(token.claims)
// }
#[derive(Debug, Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

pub async fn get_github_user(access_token: &str) -> Result<GithubUser, AppError> {
    let client = reqwest::Client::new();

    // 1. Fetch user profile
    let mut user: GithubUser = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "cpress-axum-app")
        .send()
        .await
        .map_err(|e| AppError::Unknown(e.to_string()))?
        .json()
        .await
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    // 2. Fetch emails (important part)
    let emails: Vec<GithubEmail> = client
        .get("https://api.github.com/user/emails")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "cpress-axum-app")
        .send()
        .await
        .map_err(|e| AppError::Unknown(e.to_string()))?
        .json()
        .await
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    // 3. Pick primary verified email
    let primary_email = emails
        .into_iter()
        .find(|e| e.primary && e.verified)
        .map(|e| e.email);

    user.email = primary_email;

    Ok(user)
}

pub async fn google_oauth_login(
    state: &AppState,
    code: String,
) -> Result<Option<OAuthUserLoginResponse>, AppError> {
    let id_token = get_google_oauth_token(&state.env_keys, code).await?;
    let google_claims = extract_google_claims(&id_token)?;

    let new_user: CreateOAuthUser = CreateOAuthUser {
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
    return Ok(Some(OAuthUserLoginResponse::new(user_res, jwt_token)));
}

pub async fn github_oauth_login(
    state: &AppState,
    code: String,
) -> Result<Option<OAuthUserLoginResponse>, AppError> {
    let access_token = get_github_oauth_token(&state.env_keys, code).await?;
    let github_user = get_github_user(&access_token).await?;

    let new_user: CreateOAuthUser = CreateOAuthUser {
        username: github_user.login,
        email: github_user
            .email
            .ok_or(AppError::Unknown("GitHub email not available".into()))?,
        oauth_id: github_user.id.to_string(),
    };

    // correct the database
    let user_res = GithubAuthRepository::find_or_create(&state.pool, new_user).await?;

    let jwt_token = jwt::generate_token(
        user_res.id,
        &state.env_keys.get(utils::env_registry::EnvKey::JwtSecret),
    )
    .map_err(|e| AppError::FailedToCreateJwtToken(e.to_string()))?;
    return Ok(Some(OAuthUserLoginResponse::new(user_res, jwt_token)));
}

#[derive(Serialize, Deserialize)]
pub struct AuthUserInfo {
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
