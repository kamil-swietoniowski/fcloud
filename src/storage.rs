use std::fs;
use std::io;
use std::io::Write;

pub fn database_init() {
    todo!()
}

pub fn save_user() {
    todo!()
}

pub fn get_user_by_username() {
    todo!()
}

pub fn create_file_record() {
    todo!()
}

pub fn get_file_by_id() {
    todo!()
}

pub fn append_chunk_to_file() {
    todo!()
}

pub fn update_uploaded_size() {
    todo!()
}

pub fn update_file_status() {
    todo!()
}

pub fn list_user_files() {
    todo!()
}

pub fn delete_file_from_disk_and_db() {
    todo!()
}

// pub fn list_directory(path: &str) -> io::Result<Vec<String>> {
//     let files = fs::read_dir(path)?;
//     let names = files
//         .filter_map(|entry| entry.ok())
//         .filter_map(|entry| {
//             entry
//                 .path()
//                 .file_name()
//                 .and_then(|name| name.to_str().map(|name| name.to_string()))
//         })
//         .collect();
//     Ok(names)
// }
//
//
// pub fn save_file(path: &str, body: &[u8]) -> Result<(), io::ErrorKind> {
//     if fs::File::open(path).is_ok() {
//         return Err(io::ErrorKind::AlreadyExists)
//     }
//
//     let mut file = match fs::File::create(path) {
//         Ok(t) => t,
//         Err(_) => return Err(io::ErrorKind::Other)
//     };
//
//     if file.write_all(body).is_err() {
//         return Err(io::ErrorKind::Other);
//     };
//     Ok(())
// }
