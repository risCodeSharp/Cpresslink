use crate::{error::AppError, models::links::ShortLink};
use sqlx::PgPool;

pub struct LinkRepository;

impl LinkRepository {
    pub async fn create(pool: &PgPool, user_id: i32, name: &str, slug: &str, destination: &str) -> Result<ShortLink, AppError>{
        let link: ShortLink = sqlx::query_as::<_, ShortLink>(
            "INSERT INTO shortlinks (user_id, name, slug, destination)
            VALUES ($1,$2,$3,$4)
            RETURNING id, user_id, name, slug, destination, created_at, expires_at, notified
            "
        )
        .bind(user_id)
        .bind(name)
        .bind(slug)
        .bind(destination)
        .fetch_one(pool)
        .await?;
        Ok(link)
    }

    pub async fn get_by_slug(pool: &PgPool, user_id: i32, slug: &str) -> Result<Option<ShortLink>, AppError> {
        let link: Option<ShortLink> = sqlx::query_as::<_, ShortLink>(
            "SELECT id, user_id, name, slug, destination, created_at, expires_at, notified
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

    pub async fn get_by_id(pool: &PgPool, user_id: i32, link_id: i32) -> Result<Option<ShortLink>, AppError> {
        let link: Option<ShortLink> = sqlx::query_as::<_, ShortLink>(
            "SELECT id,  user_id, name, slug, destination, created_at, expires_at, notified
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
    
    pub async fn get_by_username(pool: &PgPool, username: &str, slug: &str) -> Result<Option<ShortLink>, AppError> {
        let link: Option<ShortLink> = sqlx::query_as::<_, ShortLink>(
            r#"SELECT 
                s.id, 
                s.user_id, 
                s.name, 
                s.slug, 
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

    pub async fn list_links(pool: &PgPool, user_id: i32) -> Result<Vec<ShortLink>, AppError> {
        let links: Vec<ShortLink> = sqlx::query_as::<_, ShortLink>(
            "SELECT id, user_id, name, slug, destination, created_at, expires_at
            FROM shortlinks
            WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(links)
    }
}
