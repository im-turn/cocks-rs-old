use crate::{
    FromString,
    GetVariants,
    BIN::prompt,
};

#[derive(Debug, PartialEq)]
pub enum Shape {
    Cylindrical,
    Tapered,
    Other(String),
}

impl Shape {
    pub fn get_shape(&self) -> &str {
        match self {
            Shape::Cylindrical => "Cylindrical",
            Shape::Tapered => "Tapered",
            Shape::Other(other) => other,
        }
    }
}

impl FromString for Shape {
    fn from_string(shape: &str) -> Shape {
        match shape {
            "Cylindrical" => Shape::Cylindrical,
            "Tapered" => Shape::Tapered,
            "Other" => {
                let other = prompt("What is the shape?");
                Shape::Other(other)
            },
            _ => panic!("Invalid shape"),
        }
    }
}

impl GetVariants for Shape {
    fn get_variants() -> Vec<String> {
        vec![
            String::from("Cylindrical"),
            String::from("Tapered"),
            String::from("Other"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape() {
        let cylindrical = Shape::Cylindrical;
        let tapered = Shape::Tapered;
        let other = Shape::Other(String::from("test"));

        assert_eq!(cylindrical, Shape::Cylindrical);
        assert_eq!(tapered, Shape::Tapered);
        assert_eq!(other, Shape::Other(String::from("test")));
    }
}
