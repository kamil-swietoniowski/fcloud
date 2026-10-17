pub mod models;
pub mod database;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let _db = database::connect_to_db().await.unwrap();
    
}   


