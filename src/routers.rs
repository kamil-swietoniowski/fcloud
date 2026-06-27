use std::io;

use axum::{Json, body::Bytes, http::{HeaderMap, StatusCode}};
use crate::storage; // Korzystamy z modułu obok

pub async fn list_files_handler(headers: HeaderMap) -> Result<Json<Vec<String>>, StatusCode> {
    // 1. Wyciągamy dane z zapytania HTTP
    let path = match headers.get("path") {
        Some(t) => t.to_str().unwrap_or("."),
        None => ".",
    };

    // 2. Zlecamy czystą logikę do warstwy storage
    match storage::list_directory(path) {
        Ok(files) => Ok(Json(files)),
        Err(_) => Err(StatusCode::NOT_FOUND), // 3. Tłumaczymy błąd dysku na błąd HTTP
    }
}


pub async fn receive_file_handler(headers: HeaderMap, body: Bytes) -> StatusCode {
    let file_name = match headers.get("file_name") {
        Some(t) => t.to_str().unwrap(),
        None => return StatusCode::BAD_REQUEST,
    };
    
    match storage::save_file(file_name, &body) {
        Ok(_) => {},
       Err(io::ErrorKind::AlreadyExists) => return StatusCode::CONFLICT,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    StatusCode::OK
}
