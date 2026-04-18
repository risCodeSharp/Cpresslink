use redis::AsyncCommands;
use crate::config::state::AppState;

pub async fn push_analytics(state: &AppState, event: &str) {
    let mut connection = state.redis.get_multiplexed_async_connection().await.unwrap();
    let _: () = connection.rpush("queue:analytics", event).await.unwrap();

}

pub async fn push_email_job(state: &AppState, payload: &str) {
    let mut connection = state.redis.get_multiplexed_async_connection().await.unwrap();
    let _ : () = connection.rpush("queue:email_jobs", payload).await.unwrap();
}

