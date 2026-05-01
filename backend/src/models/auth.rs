use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
}    

#[derive(Serialize, Deserialize)]
pub struct CreateOAuthUser {
    pub username: String,
    pub email: String,
    // pub provider_name: String,
    pub oauth_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthUserLoginResponse {
    pub user: UserResponse,
    pub jwt_token: String,
}




impl OAuthUserLoginResponse {
    pub fn new(user: UserResponse, jwt_token: String) -> Self {
        Self {
            user,
            jwt_token
        }    
    }    
}    
