use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ShortLink {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub slug: String,
    pub short_code: String,
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
