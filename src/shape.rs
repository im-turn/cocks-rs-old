#[derive(Debug, PartialEq)]
pub enum Shape {
    Cylindrical,
    Tapered,
    Other(String),
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
