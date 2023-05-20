use crate::Score;

#[derive(Debug, PartialEq)]
pub enum Abnormalities {
    None,
    Minor(String),
    Major(String),
}

impl Score for Abnormalities {
    fn score(&self) -> u32 {
        match self {
            Abnormalities::None => 3,
            Abnormalities::Minor(_) => 2,
            Abnormalities::Major(_) => 1,
        }
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


#[test]
fn abnormalities_test() {
    let major_abnormality = Abnormalities::Major(String::from("major"));
    let minor_abnormality = Abnormalities::Minor(String::from("minor"));
    let no_abnormality = Abnormalities::None;

    assert_eq!(major_abnormality.get_abnormality(), "major");
    assert_eq!(minor_abnormality.get_abnormality(), "minor");
    assert_eq!(no_abnormality.get_abnormality(), "None");

    assert_eq!(major_abnormality.score(), 1);
    assert_eq!(minor_abnormality.score(), 2);
    assert_eq!(no_abnormality.score(), 3);
}
