use crate::{error::AppError, models::links::ShortLink, utils};
use sqlx::PgPool;
use tracing::info;

pub struct LinkRepository;

impl LinkRepository {
    pub async fn create(pool: &PgPool, user_id: i64, name: &str, slug: &str, destination: &str) -> Result<ShortLink, AppError>{
        let short_code = utils::id::generate_id();
        let link: ShortLink = sqlx::query_as::<_, ShortLink>(
            "INSERT INTO shortlinks (user_id, name, slug, destination, short_code)
            VALUES ($1,$2,$3,$4,$5)
            RETURNING id, user_id, name, slug, short_code, destination, created_at, expires_at, notified
            "
        )
        .bind(user_id)
        .bind(name)
        .bind(slug)
        .bind(destination)
        .bind(short_code)
        .fetch_one(pool)
        .await?;
        Ok(link)
    }

    pub async fn get_by_slug(pool: &PgPool, user_id: i64, slug: &str) -> Result<Option<ShortLink>, AppError> {
        let link: Option<ShortLink> = sqlx::query_as::<_, ShortLink>(
            "SELECT id, user_id, short_code, name, slug, destination, created_at, expires_at, notified
            FROM shortlinks 
            WHERE user_id = $1 
            AND  slug = $2"
        )
        .bind(user_id)
        .bind(slug)
        .fetch_optional(pool)
        .await?;

        Ok(link)
    }

    pub async fn get_by_id(pool: &PgPool, user_id: i64, link_id: i64) -> Result<Option<ShortLink>, AppError> {
        let link: Option<ShortLink> = sqlx::query_as::<_, ShortLink>(
            "SELECT id, user_id, name, short_code, slug, destination, created_at, expires_at, notified
            FROM shortlinks
            WHERE user_id = $1
            AND id = $2"
        )
        .bind(user_id)
        .bind(link_id)
        .fetch_optional(pool)
        .await?;

        Ok(link)
    }
    
    pub async fn get_by_short_code(pool: &PgPool, short_code: &str) -> Result<ShortLink, AppError> {
        let link: ShortLink = sqlx::query_as::<_, ShortLink>(
            "SELECT id, user_id, name, slug, short_code, destination, created_at, expires_at, notified
            FROM shortlinks
            WHERE short_code = $1"
        )
        .bind(short_code)
        .fetch_one(pool)
        .await?;
        info!("link: {:?}", link);
        Ok(link)
    }
    
    pub async fn get_by_username(pool: &PgPool, username: &str, slug: &str) -> Result<Option<ShortLink>, AppError> {
        let link: Option<ShortLink> = sqlx::query_as::<_, ShortLink>(
            r#"SELECT 
                s.id, 
                s.user_id, 
                s.name, 
                s.slug, 
                s.short_code, 
                s.destination, 
                s.created_at, 
                s.expires_at, 
                s.notified
            FROM shortlinks s
            INNER JOIN users u ON s.user_id = u.id
            WHERE u.username = $1
            AND s.slug = $2
            "#
        )
        .bind(username)
        .bind(slug)
        .fetch_optional(pool)
        .await?;

        Ok(link)
    }
 pub async fn get_email_by_link_id(pool: &PgPool, link_id: i64) -> Result<Option<String>, AppError> {
        let email: Option<String> = sqlx::query_scalar(
            r#"
            SELECT email 
            FROM users 
            WHERE id = $1
            "#,
        )
        .bind(link_id)
        .fetch_optional(pool)
        .await?;
        
        Ok(email)
    }
    pub async fn list_links(pool: &PgPool, user_id: i64) -> Result<Vec<ShortLink>, AppError> {
        let links: Vec<ShortLink> = sqlx::query_as::<_, ShortLink>(
            "SELECT id, user_id, name, slug, destination, created_at, expires_at, short_code, notified
            FROM shortlinks
            WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(links)
    }
}
