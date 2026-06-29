use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{error, json};
use thiserror::Error;

// new error type 
#[derive(Error, Debug)]
pub enum FCloudError {
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





