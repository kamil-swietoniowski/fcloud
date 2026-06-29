use std::fs;
use std::path::Path;

use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

use crate::errors::AppResult;

const DATABASE_INIT: &str = "CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS files (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    total_size INTEGER NOT NULL,
    uploaded_size INTEGER NOT NULL,
    chunk_size INTEGER NOT NULL,
    file_hash TEXT NOT NULL,
    status TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);";


pub async fn database_init() -> Result<SqlitePool, sqlx::Error> {
    let db_path = "fcloud.db";
    fs::create_dir_all("./storage").unwrap_or_default();

    if !Path::new(db_path).exists() {
        fs::File::create(db_path).map_err(|_| {
            sqlx::Error::WorkerCrashed
        })?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path)
        .await?;

    sqlx::query(DATABASE_INIT)
        .execute(&pool)
        .await?;
    Ok(pool)
}

pub async fn save_user(pool: &SqlitePool, id: &str, username: &str, password_hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)")
        .bind(id)
        .bind(username)
        .bind(password_hash)
        .execute(pool)
        .await?;

    Ok(())
}

pub fn get_user_by_username() {
    todo!()
}

pub fn create_file_record() {
    todo!()
}

pub fn get_file_by_id() {
    todo!()
}

pub fn append_chunk_to_file() {
    todo!()
}

pub fn update_uploaded_size() {
    todo!()
}

pub fn update_file_status() {
    todo!()
}

pub fn list_user_files() {
    todo!()
}

pub fn delete_file_from_disk_and_db() {
    todo!()
}

// pub fn list_directory(path: &str) -> io::Result<Vec<String>> {
//     let files = fs::read_dir(path)?;
//     let names = files
//         .filter_map(|entry| entry.ok())
//         .filter_map(|entry| {
//             entry
//                 .path()
//                 .file_name()
//                 .and_then(|name| name.to_str().map(|name| name.to_string()))
//         })
//         .collect();
//     Ok(names)
// }
//
//
// pub fn save_file(path: &str, body: &[u8]) -> Result<(), io::ErrorKind> {
//     if fs::File::open(path).is_ok() {
//         return Err(io::ErrorKind::AlreadyExists)
//     }
//
//     let mut file = match fs::File::create(path) {
//         Ok(t) => t,
//         Err(_) => return Err(io::ErrorKind::Other)
//     };
//
//     if file.write_all(body).is_err() {
//         return Err(io::ErrorKind::Other);
//     };
//     Ok(())
// }
