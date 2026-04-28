use crate::{
    error::AppError,
    models::events::ClickEvent,
    utils::{geo::GeoInfo, user_agent::UserAgentInfo},
};
use sqlx::PgPool;
use tracing::info;

pub struct AnalyticsRepository;

impl AnalyticsRepository {
    pub async fn store_click(pool: &PgPool, click: &ClickEvent) -> Result<i64, AppError> {
        let click_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO clicks(shortlink_id, referrer, clicked_at) VALUES ($1, $2, $3)
                RETURNING id",
        )
        .bind(click.link_id)
        .bind(&click.referrer)
        .bind(click.timestamp)
        .fetch_one(pool)
        .await?;
        info!("DB CALLED");
        Ok(click_id)
    }

    pub async fn store_user_agent_info(
        pool: &PgPool,
        click_id: i64,
        ua: UserAgentInfo,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO click_user_agents (click_id, browser, os, device, is_bot, bot_name)
         VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(click_id)
        .bind(ua.browser)
        .bind(ua.os)
        .bind(ua.device)
        .bind(ua.is_bot)
        .bind(ua.bot_name)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn store_geo_info(
        pool: &PgPool,
        click_id: i64,
        loc: GeoInfo,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO click_locations(click_id, country, region, city)
                VALUES($1,$2,$3,$4)",
        )
        .bind(click_id)
        .bind(loc.country)
        .bind(loc.region)
        .bind(loc.city)
        .execute(pool)
        .await?;

        Ok(())
    }
}
