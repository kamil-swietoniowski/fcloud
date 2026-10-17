pub mod models;
pub mod database;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let db = database::connect_to_db("sqlite://baza.db").await.unwrap();
    
}   


