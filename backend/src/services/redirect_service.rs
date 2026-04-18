use crate::config::state::AppState;
use crate::db::cache::link::LinkCache;
use crate::db::respository::links::LinkRepository;
use crate::error::AppError;
use crate::models::redirect::RedirectValue;

pub struct RedirectService;

impl RedirectService {
    pub async fn get_link(
        state: AppState,
        username: &str,
        slug: &str,
    ) -> Result<RedirectValue, AppError> {
        // cache lookup
        if let Some(cached) = LinkCache::get_link(&state.redis, &username, &slug).await {
            return Ok(cached);
        }

        // fallback to db
        let link = LinkRepository::get_by_username(&state.pool, &username, &slug)
            .await?
            .ok_or(AppError::NoRecordsFound)?;

        
        // update the cache to laod the link
        LinkCache::set(&state.redis, username, slug, &link.name, &link.destination ).await?;

        Ok(RedirectValue {
            destination: link.destination,
            name: link.name,
        })
    }
}
