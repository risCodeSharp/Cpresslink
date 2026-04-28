use crate::models::events::{ClickEvent, WorkerEvents};
use crate::queue::{RedisConsumer};
use crate::services::analytics_service::AnalyticsService;
use crate::utils::{self, user_agent};
use sqlx::PgPool;
use tokio::runtime::Handle;
use tracing::{debug, error, info};

pub fn run(pool: PgPool) {
    info!("[Started]: Device worker");
    let mut consumer = RedisConsumer::new(&utils::get_redis_connection_str(), WorkerEvents::Device);
    loop {
        match consumer.consume_blocking::<ClickEvent>() {
            Ok(event) => {
                debug!("[Event occurred]: {:?}", event);

                let device = user_agent::parse_user_agent(&event.user_agent);

                if let Err(e) =  Handle::current().block_on(AnalyticsService::store_user_agent_info(&pool, event.id.unwrap(), device)) {
                    error!("Failed to store device information. [Err]: {e:?}");
                }
            }
            Err(e) => {
                error!("Failed to consume in Device worker. [Err]: {e:?}");
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
