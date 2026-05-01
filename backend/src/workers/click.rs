use sqlx::PgPool;
use tracing::{error, info};

use crate::models::events::{ClickEvent, MilestoneEvent, NotificationEvent, WorkerEvents};
use crate::queue::{RedisConsumer, RedisPublisher};
use crate::services::analytics_service::AnalyticsService;
use crate::utils;

use std::{thread, time::Duration};
use tokio::runtime::Handle;

pub fn run(queue: RedisPublisher, pool: PgPool) {
    info!("[Started]: Click worker");

    let mut consumer = RedisConsumer::new(&utils::get_redis_connection_str(), WorkerEvents::Click);

    loop {
        match consumer.consume_blocking::<ClickEvent>() {
            Ok(mut event) => {
                info!("[Event occurred]: {:?}", event);
                
                // store click
                let click_id = match Handle::current()
                    .block_on(AnalyticsService::store_click(&pool, &event))
                {
                    Ok(id) => Some(id),
                    Err(e) => {
                        error!("Store failed: {:?}", e);
                        continue;
                    }
                };
                event.id = click_id;
                
                // publish geo + device events
                Handle::current().block_on(async {
                    if let Err(e) = queue.publish(WorkerEvents::Geo, &event).await {
                        error!("Failed to publish geo event: {e:?}");
                    }

                    if let Err(e) = queue.publish(WorkerEvents::Device, &event).await {
                        error!("Failed to publish device event: {e:?}");
                    }
                });
                
                // increment click counter
                let count = match Handle::current()
                    .block_on(AnalyticsService::increment_click(event.link_id))
                {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Failed to increment click: {:?}", e);
                        continue;
                    }
                };
                if matches!(count, 100 | 1000 | 10000) {
                    info!("Milestone reached: {count}");

                    if let Err(e) = Handle::current().block_on(async {
                        queue
                            .publish(
                                WorkerEvents::Notification,
                                &NotificationEvent::Milestone(MilestoneEvent {
                                    user_id: event.user_id,
                                    link_id: event.link_id,
                                    count,
                                }),
                            )
                            .await
                    }) {
                        error!("Failed to publish notification: {e:?}");
                    }
                }
            }
            Err(e) => {
                error!("Redis consume error: {e:?}");
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
