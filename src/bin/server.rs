use axum::{Router, routing::{get, post}};

#[path = "../storage.rs"]
mod storage;
#[path = "../handlers.rs"]
mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/list", get(handlers::list_files_handler))
        .route("/send", post(handlers::receive_file_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Serwer fcloud ruszył!");
    axum::serve(listener, app).await.unwrap();
}

