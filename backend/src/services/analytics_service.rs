use sqlx::{PgPool};

use crate::{db::respository::analytics::AnalyticsRepository, error::AppError, models::events::ClickEvent, utils::{geo::GeoInfo, user_agent::UserAgentInfo}};


pub struct AnalyticsService;

impl AnalyticsService {
    pub async fn store_click(pool: &PgPool, click: &ClickEvent) -> Result<i64, AppError>{
        let click_id = AnalyticsRepository::store_click(&pool, click).await?;
        Ok(click_id)
    }

    pub async fn increment_click(link_id: i64) -> Result<i64, AppError> {
        Ok(1)
    }

    pub async fn store_user_agent_info(pool: &PgPool ,click_id: i64, ua: UserAgentInfo) -> Result<(), AppError> {
        AnalyticsRepository::store_user_agent_info(&pool, click_id, ua).await?;
        Ok(())
    }
    
    pub async fn store_geo_info(pool: &PgPool, click_id: i64, geolocation: GeoInfo) -> Result<(), AppError> {
        AnalyticsRepository::store_geo_info(&pool, click_id, geolocation).await?;
        Ok(())
    }


}

