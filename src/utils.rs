use std::fs::read_to_string;

fn read_file(file_path: String) -> Result<String, std::io::Error> {
    read_to_string(file_path)
}

pub fn read_file_to_json(file_path: String) -> Result<serde_json::Value, serde_json::Error> {
    let content = read_file(file_path);
    match content {
        Ok(content) => serde_json::from_str(&content),
        Err(error) => Err(serde_json::Error::io(error)),
    }
}