// use crate::queue::{RedisPublisher};
use crate::models::events::{ClickEvent, WorkerEvents};
use crate::queue::RedisConsumer;
use crate::services::analytics_service::AnalyticsService;
use crate::utils::{self, geo};
use sqlx::PgPool;
use tokio::runtime::Handle;
// use crate::services::analytics_service;
use tracing::{debug, error, info};

pub fn run(pool: PgPool) {
    info!("[Started]: Geo worker");    
    let db = geo::init_geo_db();
    let mut consumer = RedisConsumer::new(
        &utils::get_redis_connection_str(),
        WorkerEvents::Geo
    );
    loop {
        match consumer.consume_blocking::<ClickEvent>() {
            Ok(event) => {
                debug!("[Event occurred]: {:?}", event);

                let loc = match Handle::current().block_on(geo::lookup(&db, &event.ip) ){
                    Ok(geo_info) => geo_info,
                    Err(e) => {
                        error!("Failed to get GeoInfo. [Err]: {e:?}");
                        continue;
                    }
                };
                
                if let Err(e) = Handle::current().block_on( async {AnalyticsService::store_geo_info(&pool, event.id.unwrap(), loc).await }){
                    error!("Failed to store `GeoInfo`. [Err]: {e:?}");
                }
            },
             Err(e) => {
                error!("Failed to consume in Geo worker. [Err]: {e:?}");
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
