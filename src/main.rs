mod constraint;
mod user;
use std::fs;

fn main() {
    println!("Loading example file...");

    let example_users_path = "example/example_users.toml";
    let example_shifts_path = "example/example_shifts.toml";

    match fs::read_to_string(&example_users_path) {
        Ok(loaded_file_string) => {
            let parsed_users = user::parse_users(loaded_file_string.as_str()).unwrap_or_default();

            println!("{}", example_users_path);
            for parsed_user in &parsed_users {
                println!("User:: name: {}", parsed_user.name);
            }
        }
        Err(_e) => {
            println!(
                "Failed to load {} contents. Aborting the program.",
                example_users_path
            );
            return;
        }
    }

    match fs::read_to_string(example_shifts_path) {
        Ok(loaded_file_string) => {
            let parsed_shifts = constraint::parse_shifts(loaded_file_string).unwrap_or_default();

            for (name, parsed_shift) in &parsed_shifts {
                println!(
                    "Shift:: name: {}, start: {}, end: {}",
                    name.as_str(),
                    format!("{}", parsed_shift.start),
                    format!("{}", parsed_shift.end)
                );
            }
        }
        Err(_e) => {
            println!(
                "Failed to load {} contents. Aborting the program.",
                example_shifts_path
            );
            return;
        }
    }
}
