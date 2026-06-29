use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{error, json};
use thiserror::Error;

// new error type 
#[derive(Error, Debug)]
pub enum AppError {
    // sqlite error, packing (correct word?) the raw sqlx error
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    // io error
    #[error("I/O error (disk): {0}")]
    IoError(#[from] std::io::Error),

    // next 3 are auth problems
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Wrong login or password")]
    InvalidCredentials,

    #[error("Cant access: Wrong or expired JWT")]
    Unauthorized,

    // files errors 
    #[error("File not found")]
    FileNotFound,

    #[error("Forbidden to access the file")]
    Forbidden,

    #[error("Chunk is damaged")]
    InvalidChunk,

    #[error("Sum of SHA-256 is not the same. FIle may be damaged")]
    HashMismatch
}


// some thiserror magic to automaticly change those errors to HTTP errors in Axum
// IntoResponse is trait for axum 

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // the magic begins
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()), // 500
            AppError::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()), // 500
            AppError::UserAlreadyExists => (StatusCode::CONFLICT, self.to_string()), // 409
            AppError::InvalidCredentials => (StatusCode::BAD_REQUEST, self.to_string()), // 400
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()), // 401
            AppError::FileNotFound => (StatusCode::NOT_FOUND, self.to_string()), // 404
            AppError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()), // 403
            AppError::InvalidChunk => (StatusCode::BAD_REQUEST, self.to_string()), // 400
            AppError::HashMismatch => (StatusCode::BAD_REQUEST, self.to_string()), // 400
        };
        
        let body = Json(json!({
            "error": error_message,
            "code": status.as_u16()
        }));

        (status, body).into_response()
    }
}

// alias
pub type AppResult<T> = Result<T, AppError>;

