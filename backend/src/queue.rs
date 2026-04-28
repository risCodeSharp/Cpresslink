use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};
use tracing::info;

use crate::{models::events::WorkerEvents};

#[derive(Clone)]
pub struct RedisPublisher {
    client: redis::Client
}

impl RedisPublisher {
    pub fn new(conn_str: &str) -> Self {
        let client = redis::Client::open(conn_str)
        .expect("Failed to create Redis client");

        Self { client }
    }

    pub async fn publish<T: Serialize>(
        &self,
        queue: WorkerEvents,
        event: &T,
    ) -> redis::RedisResult<()> {
        let mut connection = self.client.get_multiplexed_async_connection().await?;

        let data = serde_json::to_string(event).map_err(|e| {
            redis::RedisError::from((redis::ErrorKind::Parse, "serialize", e.to_string()))
        })?;


        let _len: i32 = connection
            .rpush(queue.as_ref(), data)
            .await?;

        Ok(())
    }
}

pub struct RedisConsumer {
    conn: redis::Connection,
    worker_queue: String
}

impl RedisConsumer {
    pub fn new(conn_str: &str, worker_event: WorkerEvents) -> Self {
        let client = redis::Client::open(conn_str).expect("Failed to create Redis client");
        
        let conn = client
        .get_connection()
        .expect("Failed to connect to Redis");
        
        
        Self {
            conn,
            worker_queue: worker_event.to_string(),
        }
    }

    pub fn consume_blocking<T: DeserializeOwned>(&mut self) -> redis::RedisResult<T> {
        let (_key, payload): (String, String) = redis::cmd("BRPOP")
        .arg(self.worker_queue.as_str())
        .arg(0)
        .query(&mut self.conn)?;

        let event: T = serde_json::from_str(&payload).map_err(|e|{
            redis::RedisError::from((
                redis::ErrorKind::Parse,
                "deserialization error",
                e.to_string(),
            ))
        })?;
        Ok(event)
    }

}

