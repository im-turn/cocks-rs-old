use crate::{GetVariants, Score, FromString, BIN::prompt};

#[derive(Debug, PartialEq)]
pub enum Abnormalities {
    None,
    Minor(String),
    Major(String),
}

impl Score for Abnormalities {
    fn score(&self) -> u32 {
        match self {
            Abnormalities::None => 5,
            Abnormalities::Minor(_) => 3,
            Abnormalities::Major(_) => 1,
        }
    }
}

impl GetVariants for Abnormalities {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("None"),
            String::from("Minor"),
            String::from("Major"),
        ]
    }
}

impl Abnormalities {
    pub fn get_abnormality(&self) -> &str {
        match self {
            Abnormalities::None => "None",
            Abnormalities::Minor(minor) => minor,
            Abnormalities::Major(major) => major,
        }
    }
}

impl FromString for Abnormalities {
    fn from_string(abnormality: &str) -> Abnormalities {
        match abnormality {
            "None" => Abnormalities::None,
            "Minor" => {
                let minor = prompt("What is the minor abnormality?");
                Abnormalities::Minor(minor)
            },
            "Major" => {
                let major = prompt("What is the major abnormality?");
                Abnormalities::Major(major)
            },
            _ => panic!("Invalid abnormality"),
        }
    }
}

impl std::fmt::Display for Abnormalities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Abnormalities::None => write!(f, "None"),
            Abnormalities::Minor(minor) => write!(f, "Minor: {}", minor),
            Abnormalities::Major(major) => write!(f, "Major: {}", major),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abnormalities_test() {
        let major_abnormality = Abnormalities::Major(String::from("major"));
        let minor_abnormality = Abnormalities::Minor(String::from("minor"));
        let no_abnormality = Abnormalities::None;

        assert_eq!(major_abnormality.get_abnormality(), "major");
        assert_eq!(minor_abnormality.get_abnormality(), "minor");
        assert_eq!(no_abnormality.get_abnormality(), "None");

        assert_eq!(major_abnormality.score(), 1);
        assert_eq!(minor_abnormality.score(), 3);
        assert_eq!(no_abnormality.score(), 5);
    }
}
