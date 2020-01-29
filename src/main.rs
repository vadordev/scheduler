mod user;
use std::fs;

fn main() {
    println!("Loading example file...");

    let loaded_file = fs::read_to_string("example/example_users.toml");

    match loaded_file {
        Ok(loaded_file_string) => {
            let parsed_users =
                user::parse_users(loaded_file_string.as_str()).unwrap_or_default();

            for parsed_user in &parsed_users {
                println!("User:: name: {}", parsed_user.name);
            }
        }
        Err(_e) => {
            println!("Failed to load the file's contents. Aborting the program.");
        }
    }
}
