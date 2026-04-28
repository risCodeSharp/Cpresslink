use chrono::Duration;
use redis::AsyncCommands;

use crate::{error::AppError, models::redirect::{RedirectSlugKey, RedirectCodeKey, RedirectValue}};

const TTL: u64 = 60 * 60 * 2; // 2 hours 

pub struct LinkCache;

impl LinkCache {
    pub async fn set(
        redis: &redis::Client,
        slug_key: RedirectSlugKey,
        code_key: RedirectCodeKey,
        value: RedirectValue
    ) -> Result<(), AppError> {
        
        let mut connection = redis
            .get_multiplexed_async_connection()
            .await
            .map_err(AppError::FailedRedisConnection)?;

        let slug_key_str = format!("shortlink:slug:{}:{}", slug_key.username, slug_key.slug);
        let code_key_str = format!("shortlink:code:{}", code_key.code);
        
        let value = serde_json::to_string(&value).map_err(|_e| AppError::DeserializeFailed("RedirectValue".into()))?;

        connection
            .set_ex::<String, String, ()>(slug_key_str, value.clone(), TTL)
            .await
            .map_err(AppError::FailedRedisConnection)?;
        
        connection
            .set_ex::<String, String, ()>(code_key_str, value, Duration::hours(2).num_seconds() as u64)
            .await
            .map_err(AppError::FailedRedisConnection)?;

        Ok(())
    }

    pub async fn set_by_slug(
        redis: &redis::Client,
        slug_key: RedirectSlugKey,
        value: RedirectValue
    ) -> Result<(), AppError> {
        
        let mut connection = redis
            .get_multiplexed_async_connection()
            .await
            .map_err(AppError::FailedRedisConnection)?;

        let slug_key_str = format!("shortlink:slug:{}:{}", slug_key.username, slug_key.slug);
        
        let value = serde_json::to_string(&value).map_err(|_e| AppError::DeserializeFailed("RedirectValue".into()))?;

        connection
            .set_ex::<String, String, ()>(slug_key_str, value, TTL)
            .await
            .map_err(AppError::FailedRedisConnection)?;

        Ok(())
    }

    pub async fn set_by_code(
        redis: &redis::Client,
        code_key: RedirectCodeKey,
        value: RedirectValue
    ) -> Result<(), AppError> {
        
        let mut connection = redis
            .get_multiplexed_async_connection()
            .await
            .map_err(AppError::FailedRedisConnection)?;

        let code_key_str = format!("shortlink:code:{}", code_key.code);
        
        let value = serde_json::to_string(&value).map_err(|_e| AppError::DeserializeFailed("RedirectValue".into()))?;
        
        connection
            .set_ex::<String, String, ()>(code_key_str, value, Duration::hours(2).num_seconds() as u64)
            .await
            .map_err(AppError::FailedRedisConnection)?;

        Ok(())
    }

    pub async fn get_by_slug(
        redis: &redis::Client,
        username: &str,
        slug: &str,
    ) -> Option<RedirectValue> {
        let mut connection = redis.get_multiplexed_async_connection().await.unwrap();

        let key = format!("shortlink:slug:{username}:{slug}");
        let data: String = connection.get(key).await.ok()?;
        
        let value: RedirectValue = serde_json::from_str(&data).ok()?;
        
        Some(value)
    }
    
    pub async fn get_by_code(
        redis: &redis::Client,
        code: &str
    ) -> Option<RedirectValue> {
        let mut connection = redis.get_multiplexed_async_connection().await.unwrap();

        
        let key = format!("shortlink:code:{code}");
        let data: String = connection.get(key).await.ok()?;

        let value: RedirectValue = serde_json::from_str(&data).ok()?;

        Some(value)
    }
}
