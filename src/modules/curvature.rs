use crate::{
    FromString,
    GetVariants,
    BIN::prompt,
};

#[derive(Debug, PartialEq)]
pub enum Curvature {
    Straight,
    Left,
    Right,
    Upwards,
    Downwards,
    Other(String),
}

impl GetVariants for Curvature {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("Straight"),
            String::from("Left"),
            String::from("Right"),
            String::from("Upwards"),
            String::from("Downwards"),
            String::from("Other"),
        ]
    }
}

impl FromString for Curvature {
    fn from_string(curvature: &str) -> Curvature {
        match curvature {
            "Straight" => Curvature::Straight,
            "Left" => Curvature::Left,
            "Right" => Curvature::Right,
            "Upwards" => Curvature::Upwards,
            "Downwards" => Curvature::Downwards,
            "Other" => {
                let other = prompt("What is the curvature?");
                Curvature::Other(other)
            },
            _ => panic!("Invalid curvature"),
        }
    }
}

impl std::fmt::Display for Curvature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Curvature::Straight => write!(f, "Straight"),
            Curvature::Left => write!(f, "Left"),
            Curvature::Right => write!(f, "Right"),
            Curvature::Upwards => write!(f, "Upwards"),
            Curvature::Downwards => write!(f, "Downwards"),
            Curvature::Other(other) => write!(f, "{}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curvature() {
        let straight = Curvature::Straight;
        let left = Curvature::Left;
        let right = Curvature::Right;
        let upwards = Curvature::Upwards;
        let downwards = Curvature::Downwards;
        let other = Curvature::Other(String::from("test"));

        assert_eq!(straight, Curvature::Straight);
        assert_eq!(left, Curvature::Left);
        assert_eq!(right, Curvature::Right);
        assert_eq!(upwards, Curvature::Upwards);
        assert_eq!(downwards, Curvature::Downwards);
        assert_eq!(other, Curvature::Other(String::from("test")));
    }
}
