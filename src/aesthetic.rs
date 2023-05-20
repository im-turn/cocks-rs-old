use crate::Score;

#[derive(Debug)]
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

