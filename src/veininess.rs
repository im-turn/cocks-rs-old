use crate::Score;

#[derive(Debug)]
pub enum Veininess {
    Invisible,
    SlightPumper,
    Normal,
    HealthyPumper,
    Juicer,
}

impl Score for Veininess {
    fn score(&self) -> u32 {
        match self {
            Veininess::Invisible => 1,
            Veininess::SlightPumper => 2,
            Veininess::Normal => 3,
            Veininess::HealthyPumper => 4,
            Veininess::Juicer => 5,
        }
    }
}


#[test]
fn test_veininess_score() {
    assert_eq!(Veininess::Invisible.score(), 1);
    assert_eq!(Veininess::SlightPumper.score(), 2);
    assert_eq!(Veininess::Normal.score(), 3);
    assert_eq!(Veininess::HealthyPumper.score(), 4);
    assert_eq!(Veininess::Juicer.score(), 5);
}
