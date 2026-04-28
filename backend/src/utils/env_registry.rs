use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum::{AsRefStr, Display, EnumIter, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, AsRefStr, Display, EnumIter)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EnvKey {
    JwtSecret,
    ClientId,
    ClientSecret,
    RedisUrl,
    DatabaseUrl,
    SenderEmail,
    SmtpUser,
    SmtpKey,
    SmtpTlsDomain,
    RedirectUri
}

#[derive(Clone)]
pub struct EnvRegistry {
    cache: HashMap<String, String>,
}

impl EnvRegistry {
    pub fn new() -> Self {
        let mut cache: HashMap<String, String> = HashMap::new();

        for key in EnvKey::iter() {
            let key = key.to_string();
            let value = std::env::var(&key).expect(&format!("{} must be set!", &key));
            cache.insert(key, value);
        }
        Self { cache: cache }
    }

    pub fn get(&self, key: EnvKey) -> String {
        self.cache
            .get(key.as_ref())
            .expect("unexpect error!")
            .clone()
    }

    pub fn get_env(key: EnvKey) -> String {
        let key = key.to_string();
        std::env::var(&key).expect(&format!("{} must be set!", &key))
    }
}
