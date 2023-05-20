use crate::Score;

#[derive(Debug, PartialEq)]
pub enum Aesthetic {
    Gross,
    UglyButUsable,
    Normal,
    PrettyNice,
    Perfect,
}

impl Score for Aesthetic {
    fn score(&self) -> u32 {
        match self {
            Aesthetic::Gross => 1,
            Aesthetic::UglyButUsable => 2,
            Aesthetic::Normal => 3,
            Aesthetic::PrettyNice => 4,
            Aesthetic::Perfect => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aesthetic_score() {
        assert_eq!(Aesthetic::Gross.score(), 1);
        assert_eq!(Aesthetic::UglyButUsable.score(), 2);
        assert_eq!(Aesthetic::Normal.score(), 3);
        assert_eq!(Aesthetic::PrettyNice.score(), 4);
        assert_eq!(Aesthetic::Perfect.score(), 5);
    }
}
