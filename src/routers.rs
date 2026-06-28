use std::io;

use axum::{Json, body::Bytes, http::{HeaderMap, StatusCode}};
use crate::{routers, storage}; 

// create user
pub async fn handle_register() {
    todo!()
}

// login, return token
pub async fn handle_login() {
    todo!()
}

// token in header, init file upload
pub async fn handle_file_init() {
    todo!()
}

// token in header, send the chunk
pub async fn handle_chunk_upload() {
    todo!()
}

// token in header, return the list of files
pub async fn handle_list_files() {
    todo!()
}

// token in header, donwload (what else to say)
pub async fn handle_download_file() {
    todo!()
}

// delete, ofc token in header
pub async fn handle_delete_file() {
    todo!()
}
//
// pub async fn list_files_handler(headers: HeaderMap) -> Result<Json<Vec<String>>, StatusCode> {
//     let path = match headers.get("path") {
//         Some(t) => t.to_str().unwrap_or("."),
//         None => ".",
//     };
//
//     match storage::list_directory(path) {
//         Ok(files) => Ok(Json(files)),
//         Err(_) => Err(StatusCode::NOT_FOUND), 
//     }
// }
//
//
// pub async fn receive_file_handler(headers: HeaderMap, body: Bytes) -> StatusCode {
//     let file_name = match headers.get("file_name") {
//         Some(t) => t.to_str().unwrap(),
//         None => return StatusCode::BAD_REQUEST,
//     };
//
//     match storage::save_file(file_name, &body) {
//         Ok(_) => {},
//         Err(io::ErrorKind::AlreadyExists) => return StatusCode::CONFLICT,
//         Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
//     };
//
//     StatusCode::OK
// }
