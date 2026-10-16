pub mod models;
pub mod database;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    database::connect_to_db().await.unwrap();
}
