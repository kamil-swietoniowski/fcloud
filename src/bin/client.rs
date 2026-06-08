use std::io::{self, Write};

#[path = "../api.rs"]
mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = input("Enter URL (leave blank for default): ");
    let url = if url.is_empty() { "http://127.0.0.1:3000".to_string() } else { url };

    let fcloud = api::FcloudClient::new(url);

    loop {
        println!("\n--- Co chcesz zrobić? ---");
        println!("1. Pokaż listę plików");
        println!("2. Wyślij plik na serwer");
        println!("3. Wyjdź");
        let choice = input("Wybierz opcję: ");

        match choice.as_str() {
            "1" => {
                let path = input("Podaj ścieżkę na serwerze (np. .): ");
                match fcloud.get_list(path).await {
                    Ok(files) => {
                        println!("\nPliki na serwerze:");
                        for file in files {
                            println!(" - {file}");
                        }
                    }
                    Err(e) => println!("Błąd pobierania listy: {e}"),
                }
            }
            "2" => {
                let local_path = input("Podaj ścieżkę do pliku lokalnego: ");
                let target_name = input("Podaj nazwę, pod jaką zapisać plik na serwerze: ");
                
                println!("Wysyłanie...");
                match fcloud.upload_file(&local_path, &target_name).await {
                    Ok(200) => println!("Sukces! Plik został wysłany."),
                    Ok(409) => println!("Błąd: Plik o takiej nazwie już istnieje na serwerze! (409 Conflict)"),
                    Ok(code) => println!("Serwer zwrócił błąd o kodzie: {code}"),
                    Err(e) => println!("Błąd wysyłania: {e}"),
                }
            }
            "3" => break,
            _ => println!("Nieprawidłowy wybór."),
        }
    }

    Ok(())
}

fn input(query: &str) -> String {
    let mut buf = String::new();
    print!("{query}");
    io::stdout().flush().expect("Problem z wypisaniem");
    io::stdin().read_line(&mut buf).expect("Problem z wczytywaniem linii");
    buf.trim().to_string()
}

