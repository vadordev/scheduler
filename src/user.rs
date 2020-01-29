use serde::Deserialize;
use toml::de::from_str;

//type Userid = i32;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}

#[derive(Deserialize)]
struct UsersToml {
    users: Vec<User>,
}

pub fn parse_users(file_contents: &str) -> Option<Vec<User>> {
    match from_str::<UsersToml>(&file_contents) {
        Ok(output_users) => Some(output_users.users),
        Err(_) => None,
    }
}
