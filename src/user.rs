#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub discord_name: String
}

#[derive(Debug, PartialEq)]
pub enum ID {
    Anonymous,
    User(User)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let user = User {
            name: String::from("S"),
            discord_name: String::from("test")
        };

        assert_eq!(user.name, "S");
        assert_eq!(user.discord_name, String::from("test"));
    }

    #[test]
    fn test_id() {
        let user = User {
            name: String::from("S"),
            discord_name: String::from("test")
        };

        let id = ID::User(user);

        match id {
            ID::User(user) => {
                assert_eq!(user.name, "S");
                assert_eq!(user.discord_name, String::from("test"));
            },
            _ => panic!("Expected ID::User")
        }
    }
}
