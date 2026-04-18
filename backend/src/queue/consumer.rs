use crate::config::state::AppState;
use redis::AsyncCommands;

pub async fn pop_analytics(state: &AppState) -> Option<String> {
    let mut connection = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .unwrap();
    connection.blpop("queue:analytics", 0.0).await.unwrap()
}

pub async fn pop_email_job(state: &AppState) -> Option<String> {
    let mut connection = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    connection.blpop("queue:email_jobs", 0.0).await.unwrap()
}