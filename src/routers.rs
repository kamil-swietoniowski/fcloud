use std::io;

use axum::{Json, body::Bytes, http::{HeaderMap, StatusCode}};
use crate::{routers, storage}; 

pub async fn handle_register() {
    todo!()
}

pub async fn handle_login() {
    todo!()
}

pub async fn handle_file_init() {
    todo!()
}

pub async fn handle_chunk_upload() {
    todo!()
}

pub async fn handle_list_files() {
    todo!()
}

pub async fn handle_download_file() {
    todo!()
}

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
