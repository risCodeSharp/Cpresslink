use tracing::info;

use crate::config::state::AppState;
use crate::db::cache::link::LinkCache;
use crate::db::respository::links::LinkRepository;
use crate::error::AppError;
use crate::models::events::{ClickEvent, WorkerEvents};
use crate::models::redirect::{RedirectCodeKey, RedirectSlugKey, RedirectValue};

pub struct RedirectService;

impl RedirectService {
    

    pub async fn get_by_slug(
        state: AppState,
        username: &str,
        slug: &str,
        ip_addr: String,
        user_agent: String,
        refer: Option<String>,
    ) -> Result<RedirectValue, AppError> {
        // cache lookup
        if let Some(cached) = LinkCache::get_by_slug(&state.redis, &username, &slug).await {
            let event = ClickEvent::new(cached.link_id, cached.user_id, ip_addr, refer ,user_agent);
            let _ = state
                .redis_publisher
                .publish(WorkerEvents::Click, &event)
                .await;
            return Ok(cached);
        }

        // fallback to db
        let link = LinkRepository::get_by_username(&state.pool, &username, &slug)
            .await?
            .ok_or(AppError::NoRecordsFound)?;

        let event = ClickEvent::new(link.id, link.user_id, ip_addr, refer, user_agent);
        let _ = state
            .redis_publisher
            .publish(WorkerEvents::Click, &event)
            .await;

        let value = RedirectValue::new(link.destination.to_string(), link.id, link.user_id);
        // load into cache for faster for next time
        LinkCache::set_by_slug(
            &state.redis,
            RedirectSlugKey::new(username.to_string(), slug.to_string()),
            value.clone(),
        )
        .await?;

        Ok(value)
    }

    pub async fn get_by_code(state: AppState, code: String, ip_addr: String, user_agent: String, refer: Option<String>) -> Result<RedirectValue, AppError> {
        // cache lookup
        if let Some(cached) = LinkCache::get_by_code(&state.redis, &code).await {
            let event = ClickEvent::new(cached.link_id, cached.user_id, ip_addr, refer, user_agent);
            let _ = state
                .redis_publisher
                .publish(WorkerEvents::Click, &event)
                .await;
            return Ok(cached);
        }

        // fallback to db
        let link = LinkRepository::get_by_short_code(&state.pool, &code)
            .await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        info!("{:?}", link);
        let event = ClickEvent::new(link.id, link.user_id, ip_addr, refer, user_agent);
        let _ = state
            .redis_publisher
            .publish(WorkerEvents::Click, &event)
            .await;

        let value = RedirectValue::new(link.destination.to_string(), link.id, link.user_id);
        // load into cache for faster for next time
        LinkCache::set_by_code(
            &state.redis,
            RedirectCodeKey::new(code.to_string()),
            value.clone(),
        )
        .await?;
        Ok(value)
    }

     
}
