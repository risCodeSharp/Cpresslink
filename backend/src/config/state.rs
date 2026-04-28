use redis::Client;
use sqlx::{PgPool, Pool, Postgres};

use crate::{queue::RedisPublisher, utils::env_registry::EnvRegistry};

#[derive(Clone)]
pub struct AppState {
    pub redis: Client,
    pub redis_publisher: RedisPublisher,
    pub pool: Pool<Postgres>,
    pub env_keys: EnvRegistry,
}

impl AppState {
    pub async fn new() -> Self {
        let redis_url =
            std::env::var("REDIS_URL").expect("'REDIS_URL' env key is not found in .env!");
        let redis_publisher = RedisPublisher::new(&redis_url);
        let client = Client::open(redis_url).expect("Failed to Open Redis Client!");

        let db_connection_url =
            std::env::var("DATABASE_URL").expect("'DATABASE_URL' env key is not found in .env!");
        let pool = PgPool::connect(&db_connection_url).await.expect("Failed to connect to Postgres");

        let env_keys = EnvRegistry::new();

        Self {
            redis: client,
            redis_publisher,
            pool,
            env_keys,
        }
    }
}
