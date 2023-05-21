use crate::{
    FromString,
    GetVariants,
};

#[derive(Debug, PartialEq)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl GetVariants for Tier {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("S"),
            String::from("A"),
            String::from("B"),
            String::from("C"),
            String::from("D"),
            String::from("E"),
            String::from("F"),
        ]
    }
}

impl FromString for Tier {
    fn from_string(tier: &str) -> Tier {
        match tier {
            "S" => Tier::S,
            "A" => Tier::A,
            "B" => Tier::B,
            "C" => Tier::C,
            "D" => Tier::D,
            "E" => Tier::E,
            "F" => Tier::F,
            _ => panic!("Invalid tier"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier() {
        let s = Tier::S;
        let a = Tier::A;
        let b = Tier::B;
        let c = Tier::C;
        let d = Tier::D;
        let e = Tier::E;
        let f = Tier::F;

        assert_eq!(s, Tier::S);
        assert_eq!(a, Tier::A);
        assert_eq!(b, Tier::B);
        assert_eq!(c, Tier::C);
        assert_eq!(d, Tier::D);
        assert_eq!(e, Tier::E);
        assert_eq!(f, Tier::F);
    }
}
