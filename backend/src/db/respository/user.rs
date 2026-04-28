use sqlx::PgPool;

use crate::{error::AppError, models::auth::User};
pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<Option<User>, AppError> {
        let user: Option<User> = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3) 
            RETURNING id, username,  email, password_hash, created_at",
        )
        .bind(username)
        .bind(email)
        .bind(password)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_username_by_id(pool: &PgPool, id: i64) -> Result<Option<String>, AppError> {
        let username: Option<String> = sqlx::query_scalar(
            r#"
                SELECT username 
                FROM users
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(username)
    }
   
    pub async fn get_email_by_id(pool: &PgPool, link_id: i64) -> Result<Option<String>, AppError> {
        let email: Option<String> = sqlx::query_scalar(
            r#"
            SELECT u.email 
            FROM users u
            JOIN shortlinks s ON u.id = s.user_id
            WHERE s.id = $1
            "#,
        )
        .bind(link_id)
        .fetch_optional(pool)
        .await?;
        
        Ok(email)
    }
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, created_at 
            FROM users
            WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}
