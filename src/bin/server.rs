use axum::{Router, extract::path, routing::{get, post}};

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
        .route("/list", get(routers::list_files_handler))
        .route("/send", post(routers::receive_file_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Serwer fcloud ruszył!");
    axum::serve(listener, app).await.unwrap();
}

