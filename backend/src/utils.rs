pub mod jwt;
pub mod response;
pub mod email;
pub mod email_templates;
pub mod geo;
pub mod user_agent;
pub mod id;
pub mod logger;
pub mod env_registry;

pub fn get_redis_connection_str() -> String {
    std::env::var("REDIS_URL").expect("Failed to load `REDIS_URL` env key")
}