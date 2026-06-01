use reqwest::{Client, header};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = input("Enter URL(leave blank for default): ");
    let url = if url.is_empty() { "http://127.0.0.1:3000/list".to_string()} else {url};

    let path = input("Enter path: ");

    println!("{url}");
    let client = Client::new();
    let files: Vec<String> = client.get(url)
        .header("path", path).send().await.unwrap().json().await.unwrap();
    
    
    
    for file in files {
        println!("{file}")
    }
    Ok(())
}

use std::io::{self, Write};
fn input(query: &str) -> String {
    let mut buf = String::new();
    print!("{query}");
    io::stdout().flush().expect("Problem z wypisaniem");
    io::stdin().read_line(&mut buf).expect("Problem z wczytywaniem linii");
    buf.trim().to_string()
}
