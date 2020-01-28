use std::fs::read_to_string;
use std::io;

pub fn load_file_contents(file_path: &str) -> Result<String, io::Error> {
    read_to_string(&file_path)
}
