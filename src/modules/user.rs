use crate::{
    FromString,
    GetVariants,
};

#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub discord_name: String,
}

#[derive(Debug, PartialEq)]
pub enum ID {
    Anonymous,
    User(User),
}

impl GetVariants for ID {
    fn get_variants() -> Vec<String> {
        vec![String::from("Anonymous"), String::from("User")]
    }
}

impl FromString for ID {
    fn from_string(id: &str) -> ID {
        match id {
            "Anonymous" => ID::Anonymous,
            "User" => ID::User(User {
                name: String::from(""),
                discord_name: String::from(""),
            }),
            _ => panic!("Invalid ID"),
        }
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ID::Anonymous => write!(f, "Anonymous"),
            ID::User(user) => write!(f, "User: {}, Discord: {}", user.name, user.discord_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let user = User {
            name: String::from("S"),
            discord_name: String::from("test"),
        };

        assert_eq!(user.name, "S");
        assert_eq!(user.discord_name, String::from("test"));
    }

    #[test]
    fn test_id() {
        let user = User {
            name: String::from("S"),
            discord_name: String::from("test"),
        };

        let id = ID::User(user);

        match id {
            ID::User(user) => {
                assert_eq!(user.name, "S");
                assert_eq!(user.discord_name, String::from("test"));
            }
            _ => panic!("Expected ID::User"),
        }
    }
}
