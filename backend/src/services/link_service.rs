use crate::config::state::AppState;
use crate::db::cache::link::LinkCache;
use crate::db::respository::user::UserRepository;
use crate::error::AppError;
use crate::models::links::ShortLink;
use crate::models::redirect::{RedirectCodeKey, RedirectSlugKey, RedirectValue};
use crate::{db::respository::links::LinkRepository, models::links::CreateLinkRequest};
use sqlx::PgPool;

pub struct LinkService;

impl LinkService {
    pub async fn create(
        state: &AppState,
        payload: CreateLinkRequest,
        user_id: i64,
    ) -> Result<ShortLink, AppError> {
        let link_future = LinkRepository::create(
            &state.pool,
            user_id.clone(),
            &payload.name,
            &payload.slug,
            &payload.url,
        );
        let username_future = UserRepository::get_username_by_id(&state.pool, user_id);

        let (link_res, username_res) = tokio::join!(link_future, username_future);

        let link = link_res?;
        let username = username_res?.ok_or(AppError::NoUserFound)?;
        LinkCache::set(
            &state.redis,
            RedirectSlugKey::new(username, link.slug.clone()),
            RedirectCodeKey::new(link.short_code.clone()),
            RedirectValue::new(link.destination.clone(), link.id.clone(), link.user_id.clone()),
        )
        .await?;

        Ok(link)
    }

    pub async fn get_link_by_slug(
        pool: &PgPool,
        user_id: i64,
        slug: &str,
    ) -> Result<Option<ShortLink>, AppError> {
        LinkRepository::get_by_slug(&pool, user_id, &slug).await
    }
    pub async fn get_link_by_id(
        pool: &PgPool,
        user_id: i64,
        link_id: i64,
    ) -> Result<Option<ShortLink>, AppError> {
        LinkRepository::get_by_id(&pool, user_id, link_id).await
    }

    pub async fn list_links(pool: &PgPool, user_id: i64) -> Result<Vec<ShortLink>, AppError> {
        LinkRepository::list_links(&pool, user_id).await
    }
}
