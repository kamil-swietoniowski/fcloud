use axum::{
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(client_choose))
        .route("/list", get(list_files));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn client_choose(headers: HeaderMap) -> Result<String, StatusCode> {
    let client = if let Some(cl) = headers.get("client-type") {
        if cl == "r-client" {
            return Ok("Witaj R Client".to_string());
        } else {
            return Ok(format!("Nie rozpoznano: {}", cl.to_str().unwrap()));
        } 
    } else {
        return Ok("Witaj z przeglądarki!".to_string());
    };
}


async fn list_files(headers: HeaderMap) -> Result<Json<Vec<String>>, StatusCode> {
    let path = match headers.get("path") {
        Some(t) => t.to_str().unwrap(),
        None => ".",
    };
    Ok(Json(match get_files_as_strings(path) {
        Ok(t) => t,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    }))
}



use std::fs;

fn get_files_as_strings(path: &str) -> std::io::Result<Vec<String>> {
    let files = fs::read_dir(path)?;
    let names: Vec<String> = files
        .filter_map(|entry| entry.ok()) // Skip errors
        .filter_map(|entry| {
            entry
                .path()
                .file_name()
                .and_then(|name| name.to_str().map(|name| name.to_string()))
        })
        .collect();
    Ok(names)
}
