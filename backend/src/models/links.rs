use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct ShortLink {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub slug: String,
    pub destination: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub notified: bool,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateLinkRequest {
    pub name: String,
    pub url: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct RegisterLinkResponse {
    pub message: String,
}
