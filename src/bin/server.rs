use axum::{Router, routing::{delete, get, post}};

#[path ="../models.rs"]
mod models;
#[path = "../storage.rs"]
mod storage;
#[path = "../routers.rs"]
mod routers;
#[path = "../services/auth_service.rs"]
mod auth_service;
#[path = "../services/file_service.rs"]
mod file_service;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/auth/register", post(routers::handle_register))
        .route("/auth/login", post(routers::handle_login))
        .route("/files", get(routers::handle_list_files))
        .route("/files/init", post(routers::handle_file_init))
        .route("/files/upload", post(routers::handle_chunk_upload))
        .route("/files/{id}", get(routers::handle_download_file))
        .route("/files/{id}", delete(routers::handle_delete_file));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("FCloud has started");
    axum::serve(listener, app).await.unwrap();
}

