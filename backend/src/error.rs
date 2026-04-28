use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Email already  exists")]
    EmailAlreadyExists,

    #[error("No Record is found")]
    NoRecordsFound,
    
    #[error("Failed to {0} deserialize the object")]
    DeserializeFailed(String),

    #[error("No User is found")]
    NoUserFound,

    #[error("Invalid credentials")]
InvalidCredentials,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Unexpected error: {0}")]
    Unknown(String),

    #[error("Unauthorized")]
    Unauthorized(Option<String>),

    #[error("Failed connection to redis")]
    FailedRedisConnection(#[from] redis::RedisError),

    #[error("Invalid of sending email message {0}")]
    InvalidSendEmailMessage(String),

    #[error("Invalid Send To/Receiver Email")]
    InvalidToSendEmail(String),

    #[error("Failed to Send Email. {0}")]
    FailedToSendMail(String),

    #[error("Failed to create JWT token. [Err]: {0}")]
    FailedToCreateJwtToken(String),

    #[error("Wrong token")]
    InvalidToken,
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NoRecordsFound,
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("2305") => {
                AppError::EmailAlreadyExists
            }
            sqlx::Error::Database(db_err) => AppError::DatabaseError(db_err.to_string()),
            other => AppError::Unknown(other.to_string()),
        }
    }
}
