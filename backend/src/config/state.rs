use redis::Client;
use sqlx::{PgPool, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub redis: Client,
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new() -> Self {
        let redis_url =
            std::env::var("REDIS_URL").expect("'REDIS_URL' env key is not found in .env!");
        let client = Client::open(redis_url).expect("Failed to Open Redis Client!");

        let db_connection_url =
            std::env::var("DATABASE_URL").expect("'DATABASE_URL' env key is not found in .env!");
        let pool = PgPool::connect(&db_connection_url).await.expect("Failed to connect to Postgres");

        let jwt_secret: String =
            std::env::var("JWT_SECRET").expect("'JWT_SECRET' env key is not found in .env!");

        Self {
            redis: client,
            pool,
            jwt_secret,
        }
    }
}
