use crate::Score;

#[derive(Debug)]
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
