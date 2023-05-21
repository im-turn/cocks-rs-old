use crate::{GetVariants, Score, FromString};

#[derive(Debug, PartialEq)]
pub enum Balls {
    NonExistant,
    Tiny,
    Normal,
    BigSwingers,
    PossibleCancer,
}

impl Score for Balls {
    fn score(&self) -> u32 {
        match self {
            Balls::NonExistant => 2,
            Balls::Tiny => 3,
            Balls::Normal => 4,
            Balls::BigSwingers => 5,
            Balls::PossibleCancer => 1,
        }
    }
}

impl GetVariants for Balls {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("NonExistant"),
            String::from("Tiny"),
            String::from("Normal"),
            String::from("BigSwingers"),
            String::from("PossibleCancer"),
        ]
    }
}

impl FromString for Balls {
    fn from_string(balls: &str) -> Balls {
        match balls {
            "NonExistant" => Balls::NonExistant,
            "Tiny" => Balls::Tiny,
            "Normal" => Balls::Normal,
            "BigSwingers" => Balls::BigSwingers,
            "PossibleCancer" => Balls::PossibleCancer,
            _ => panic!("Invalid balls"),
        }
    }
}

impl std::fmt::Display for Balls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Balls::NonExistant => write!(f, "NonExistant"),
            Balls::Tiny => write!(f, "Tiny"),
            Balls::Normal => write!(f, "Normal"),
            Balls::BigSwingers => write!(f, "BigSwingers"),
            Balls::PossibleCancer => write!(f, "PossibleCancer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balls_score() {
        assert_eq!(Balls::NonExistant.score(), 2);
        assert_eq!(Balls::Tiny.score(), 3);
        assert_eq!(Balls::Normal.score(), 4);
        assert_eq!(Balls::BigSwingers.score(), 5);
        assert_eq!(Balls::PossibleCancer.score(), 1);
    }
}
