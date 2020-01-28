use std::io;
use std::fs::read_to_string;

pub fn load_file_contents( file_path: &str ) -> Result< String, io::Error > {
    return read_to_string( &file_path )
}