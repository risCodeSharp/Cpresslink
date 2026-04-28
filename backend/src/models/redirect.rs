use redis::{FromRedisValue, ParsingError};
use serde::{Deserialize, Serialize};

pub struct RedirectSlugKey {
    pub username: String,
    pub slug: String,
}

impl RedirectSlugKey {
    pub fn new(username: String, slug: String) -> Self {
        Self { username, slug }
    }
}

#[derive(Clone)]
pub struct RedirectCodeKey {
    pub code: String,
}

impl RedirectCodeKey {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct RedirectValue {
    pub destination: String,
    pub link_id: i64,
    pub user_id: i64,
}

impl RedirectValue {
    pub fn new(destination: String,link_id: i64,user_id: i64) -> Self {
        Self {
            destination,
            link_id,
            user_id
        }
    }
}

impl FromRedisValue for RedirectValue {
    fn from_redis_value(v: redis::Value) -> Result<Self, ParsingError> {
        let s: String = FromRedisValue::from_redis_value(v)?;
        let parsed =
            serde_json::from_str(&s).map_err(|_| ParsingError::from("JSON parsing error"))?;
        Ok(parsed)
    }
}
