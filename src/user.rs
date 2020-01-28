use serde::Deserialize;
use toml::de::from_str;

//type Userid = i32;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}

#[derive(Deserialize)]
struct UserRoot {
    users: Vec<User>,
}

pub fn read_input_users(file_contents: &str) -> Option<Vec<User>> {
    let output_users = from_str::<UserRoot>(&file_contents);
    return match output_users {
        Ok(t) => Some(t.users),
        Err(_e) => None,
    };
}
