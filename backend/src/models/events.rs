use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, AsRefStr, Display)]
pub enum WorkerEvents {
    #[strum(serialize = "geo_event")]
    Geo,
    #[strum(serialize = "click_event")]
    Click,
    #[strum(serialize = "device_event")]
    Device,
    #[strum(serialize = "notification_event")]
    Notification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClickEvent {
    pub id: Option<i64>,
    pub link_id: i64,
    pub user_id: i64,
    pub referrer: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub ip: String,
    pub user_agent: String,
}

impl ClickEvent {
    pub fn new(link_id: i64, user_id: i64, ip: String,referrer: Option<String>, user_agent: String) -> Self {
        Self {
            id: None,
            link_id,
            user_id,
            timestamp: Utc::now(),
            ip,
            referrer,
            user_agent,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MilestoneEvent {
    pub user_id: i64,
    pub link_id: i64,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreatedEvent {
    pub user_id: i64,
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NotificationEvent {
    UserCreated(UserCreatedEvent),
    Milestone(MilestoneEvent),
}
