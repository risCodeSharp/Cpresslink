
use redis::{FromRedisValue, ParsingError};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RedirectValue {
    pub destination: String,
    pub name: String,
}
impl FromRedisValue for RedirectValue {
    fn from_redis_value(v: redis::Value) -> Result<Self, ParsingError> {
        let s: String = FromRedisValue::from_redis_value(v)?;
        let parsed =
            serde_json::from_str(&s).map_err(|_| ParsingError::from("JSON parsing error"))?;
        Ok(parsed)
    }
}