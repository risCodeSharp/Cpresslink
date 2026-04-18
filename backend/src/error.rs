use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Email already  exists")]
    EmailAlreadyExists,

    #[error("No Record is found")]
    NoRecordsFound,

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
    FailedRedisConnection(#[from] redis::RedisError)
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
