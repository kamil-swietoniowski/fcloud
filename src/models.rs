use serde::{Deserialize, Serialize};
use sqlx::FromRow;


// users in db 
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

// files in db
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct FileRecord {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub path: String,
    pub total_size: i64,
    pub uploaded_size: i64,
    pub file_hash: String,
    pub status: String,
}

// JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // in this case it will be user id
    pub exp: u64, // when expires
}

// Network communication


// register / login 
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

// login success, token
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

// init file upload 
#[derive(Debug, Deserialize)] 
pub struct FileInitRequest {
    pub name: String,
    pub path: String,
    pub total_sizew: i64,
    pub chunk_size: i64,
    pub file_hash: String,
}

// init file upload response 
#[derive(Debug, Serialize)]
pub struct FileInitResponse {
    pub file_id: String,
    pub status: String,
}

