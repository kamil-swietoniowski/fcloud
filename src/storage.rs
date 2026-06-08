use std::fs;
use std::io;
use std::io::Write;

pub fn list_directory(path: &str) -> io::Result<Vec<String>> {
    let files = fs::read_dir(path)?;
    let names = files
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .path()
                .file_name()
                .and_then(|name| name.to_str().map(|name| name.to_string()))
        })
        .collect();
    Ok(names)
}


pub fn save_file(path: &str, body: &[u8]) -> Result<(), io::ErrorKind> {
    if fs::File::open(path).is_ok() {
        return Err(io::ErrorKind::AlreadyExists)
    }

    let mut file = match fs::File::create(path) {
        Ok(t) => t,
        Err(_) => return Err(io::ErrorKind::Other)
    };

    if file.write_all(body).is_err() {
        return Err(io::ErrorKind::Other);
    };
    Ok(())
}
