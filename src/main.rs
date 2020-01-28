mod user;
mod utils;

fn main() {
    println!("Loading example file...");

    let loaded_file = utils::load_file_contents("example/example_users.toml");

    match loaded_file {
        Ok(loaded_file_string) => {
            let parsed_users =
                user::read_input_users(loaded_file_string.as_str()).unwrap_or(Vec::new());

            for parsed_user in &parsed_users {
                println!("User:: name: {}", parsed_user.name);
            }
        }
        Err(_e) => {
            println!("Failed to load the file's contents. Aborting the program.");
            return;
        }
    }
}
