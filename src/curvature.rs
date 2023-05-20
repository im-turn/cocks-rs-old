#[derive(Debug, PartialEq)]
pub enum Curvature {
    Straight,
    Left,
    Right,
    Upwards,
    Downwards,
    Other(String),
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
