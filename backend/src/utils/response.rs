use axum::http::StatusCode;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;


#[derive(Serialize)]
pub struct Meta {
    pub request_id: String,
    pub timestamp: String,
    pub version: String,
}

/*  we are using v4 -> which give us random ids */

impl Meta {
    fn new(version_name: &str) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            // This converts into an RFC 3339 formated string
            // which is widely used standard for timestamp(in APIs, Json, etc)
            timestamp: Utc::now().to_rfc3339(),
            version: version_name.to_string(),
        }
    }
}

// Standard APi Response Wapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String, // "success" or "error"
    pub code: u16,
    pub data: Option<T>,
    pub message: Option<String>,
    pub meta: Meta,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, status: StatusCode) -> Self {
        ApiResponse {
            status: String::from("success"),
            code: status.as_u16(),
            data: Some(data),
            message: None,
            meta: Meta::new("v1"),
        }
    }

    pub fn error(msg: impl Into<String>, status: StatusCode) -> Self {
        ApiResponse {
            status: String::from("error"),
            code: status.as_u16(),
            data: None,
            message: Some(msg.into()),
            meta: Meta::new("v1"),
        }
    }
}


// impl ApiResponse<()> {
//     pub fn error(msg: impl Into<String>, status: StatusCode) -> Self {
//         ApiResponse {
//             status: String::from("error"),
//             code: status.as_u16(),
//             data: None,
//             message: Some(msg.into()),
//             meta: Meta::new("v1"),
//         }
//     }
// }