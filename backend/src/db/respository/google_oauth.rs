use sqlx::PgPool;
use tracing::info;

use crate::{
    error::AppError,
    models::auth::{CreateGoogleOauthUser, UserResponse},
};

const GOOGLE_PROVIDER: &str = "google";

pub struct GoogleAuthRepository;

impl GoogleAuthRepository {
    pub async fn create(
        pool: &PgPool,
        payload: &CreateGoogleOauthUser,
    ) -> Result<UserResponse, AppError> {
        let created_user = sqlx::query_as::<_, UserResponse>(
            "INSERT INTO users(oauth_provider, oauth_id, username, email) 
            values($1, $2, $3, $4) 
            RETURNING id, username, email, created_at",
        )
        .bind(GOOGLE_PROVIDER)
        .bind(&payload.oauth_id)
        .bind(&payload.username)
        .bind(&payload.email)
        .fetch_one(pool)
        .await?;

        Ok(created_user)
    }

    pub async fn exists(pool: &PgPool, oauth_id: &str) -> Result<bool, AppError> {
        let user_exists: bool = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT 1 
                FROM users 
                WHERE 
                oauth_id = $1
                AND oauth_provider = $2
                )",
        )
        .bind(oauth_id)
        .bind(GOOGLE_PROVIDER)
        .fetch_one(pool)
        .await?;
        Ok(user_exists)
    }

    pub async fn get(pool: &PgPool, oauth_id: &str) -> Result<Option<UserResponse>, AppError> {
        let user = sqlx::query_as::<_, UserResponse>(
            "SELECT id, username, email, created_at
            FROM users
            WHERE 
               oauth_id = $1
               AND oauth_provider = $2",
        )
        .bind(oauth_id)
        .bind(GOOGLE_PROVIDER)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_or_create(pool: &PgPool, payload: CreateGoogleOauthUser) -> Result<UserResponse, AppError>{
        info!("{:?}", &payload.oauth_id);
        if Self::exists(&pool, &payload.oauth_id).await? {
            let user =  Self::get(&pool, &payload.oauth_id).await?.unwrap();
            return Ok(user);
        }
        let user =  Self::create(&pool, &payload).await?;

        Ok(user)
    }

    // pub async fn get_by_email(email: &str) ->  Result<Option<UserResponse>, AppError> {

    //     Ok(())
    // }
}
