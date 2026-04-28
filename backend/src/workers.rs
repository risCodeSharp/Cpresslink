use sqlx::PgPool;

use crate::queue::{RedisPublisher};

pub mod click;
pub mod geo;
pub mod device;
pub mod notification;

pub async fn start_all(publisher: RedisPublisher, pool: PgPool) {
    let pool_clone = pool.clone();
    tokio::task::spawn_blocking({
        move || click::run(publisher.clone(), pool_clone)
    });
    let pool_clone = pool.clone();
    tokio::task::spawn_blocking({
        move || geo::run(pool_clone)
    });
    
    let pool_clone = pool.clone();
    tokio::task::spawn_blocking({
        move || device::run(pool_clone)
    });
    
    // let queue_clone = queue.clone();
    tokio::spawn(async move {
        notification::run( pool.clone()).await;
    });
}
