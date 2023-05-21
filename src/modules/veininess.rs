use crate::{GetVariants, Score, FromString};

#[derive(Debug, PartialEq)]
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

impl GetVariants for Veininess {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("Invisible"),
            String::from("SlightPumper"),
            String::from("Normal"),
            String::from("HealthyPumper"),
            String::from("Juicer"),
        ]
    }
}

impl FromString for Veininess {
    fn from_string(veininess: &str) -> Veininess {
        match veininess {
            "Invisible" => Veininess::Invisible,
            "SlightPumper" => Veininess::SlightPumper,
            "Normal" => Veininess::Normal,
            "HealthyPumper" => Veininess::HealthyPumper,
            "Juicer" => Veininess::Juicer,
            _ => panic!("Invalid veininess"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veininess_score() {
        assert_eq!(Veininess::Invisible.score(), 1);
        assert_eq!(Veininess::SlightPumper.score(), 2);
        assert_eq!(Veininess::Normal.score(), 3);
        assert_eq!(Veininess::HealthyPumper.score(), 4);
        assert_eq!(Veininess::Juicer.score(), 5);
    }
}
