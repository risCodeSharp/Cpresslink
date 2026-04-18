use chrono::Duration;
use redis::AsyncCommands;

use crate::{error::AppError, models::redirect::RedirectValue};

pub struct LinkCache;

impl LinkCache {
    pub async fn set(
        redis: &redis::Client,
        username: &str,
        slug: &str,
        name: &str,
        url: &str,
    ) -> Result<(), AppError> {
        let mut connection = redis
            .get_multiplexed_async_connection()
            .await
            .map_err(AppError::FailedRedisConnection)?;

        let key = format!("shortlink:{username}:{slug}");
        let value = RedirectValue {
            name: name.to_string(),
            destination: url.to_string(),
        };
        let value = serde_json::to_string(&value).unwrap();

        connection
            .set_ex::<String, String, ()>(key, value, Duration::hours(2).num_seconds() as u64)
            .await
            .map_err(AppError::FailedRedisConnection)?;
        Ok(())
    }

    pub async fn get_link(
        redis: &redis::Client,
        username: &str,
        slug: &str,
    ) -> Option<RedirectValue> {
        let mut connection = redis.get_multiplexed_async_connection().await.unwrap();

        let key = format!("shortlink:{username}:{slug}");
        let data: String = connection.get(key).await.ok()?;

        let value: RedirectValue = serde_json::from_str(&data).ok()?;

        Some(value)
    }
}
