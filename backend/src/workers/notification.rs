use crate::db::respository::user::UserRepository;
use crate::models::events::{NotificationEvent, WorkerEvents};
use crate::queue::RedisConsumer;
use crate::utils::email_templates::EmailMessage;
use crate::utils::{self, email, email_templates};

use sqlx::PgPool;
use tracing::{debug, error, info};

use std::sync::{Arc, Mutex};
use std::time::Duration;


fn send_email(
    email_addr: &str,
    msg: EmailMessage,
) -> tokio::time::Timeout<impl Future<Output = Result<(), crate::error::AppError>>> {
    tokio::time::timeout(
        Duration::from_secs(10),
        email::send(&email_addr, msg.subject, msg.text_body, msg.html_body),
    )
}

pub async fn run(pool: PgPool) {
    info!("[Started]: Notification worker");

    // shared consumer (safe across threads)
    let consumer = Arc::new(Mutex::new(RedisConsumer::new(
        &utils::get_redis_connection_str(),
        WorkerEvents::Notification,
    )));

    loop {
        let consumer_clone = consumer.clone();

        // run blocking redis in thread pool
        let event_result = tokio::task::spawn_blocking(move || {
            let mut guard = consumer_clone.lock().unwrap();
            guard.consume_blocking::<NotificationEvent>()
        })
        .await;

        // handle redis result
        let event = match event_result {
            Ok(Ok(ev)) => ev,
            Ok(Err(err)) => {
                error!("Redis consume error: {err:?}");
                continue;
            }
            Err(e) => {
                error!("Spawn task failed: {e:?}");
                continue;
            }
        };

        debug!("[Event received]: {:?}", event);

        match event {
            // When new user is created
            NotificationEvent::UserCreated(data) => {
                let username = match UserRepository::get_username_by_id(&pool, data.user_id).await {
                    Ok(Some(v)) => v,
                    Ok(None) => {
                        error!("Username not found");
                        continue;
                    }
                    Err(e) => {
                        error!("DB error: {e:?}");
                        continue;
                    }
                };

                let body = email_templates::welcome_email(&username);

                let send_result = send_email(&data.email, body)
                .await;

                match send_result {
                    Ok(Ok(_)) => {}
                    Ok(Err(e)) => error!("Email send error: {e:?}"),
                    Err(_) => error!("Email timeout"),
                }
            }

            // When a milestone is reached
            NotificationEvent::Milestone(data) => {
                let username = match UserRepository::get_username_by_id(&pool, data.user_id).await {
                    Ok(Some(v)) => v,
                    Ok(None) => {
                        error!("Username not found");
                        continue;
                    }
                    Err(e) => {
                        error!("DB error: {e:?}");
                        continue;
                    }
                };

                let body = email_templates::milestone_email(&username, data.count);

                let email_addr = match UserRepository::get_email_by_id(&pool, data.user_id).await {
                    Ok(Some(email)) => email,
                    Ok(None) => {
                        error!("Email not found");
                        continue;
                    }
                    Err(e) => {
                        error!("DB error: {e:?}");
                        continue;
                    }
                };

                let send_result = send_email(&email_addr, body).await;

                match send_result {
                    Ok(Ok(_)) => {}
                    Ok(Err(e)) => error!("Email send error: {e:?}"),
                    Err(_) => error!("Email timeout"),
                }
            }
        }
    }
}
