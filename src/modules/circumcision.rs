use crate::{
    FromString,
    GetVariants,
};

#[derive(Debug, PartialEq)]
pub enum Circumcision {
    Circumcised,
    Uncircumcised,
}

impl GetVariants for Circumcision {
    fn get_variants() -> Vec<String> {
        vec![String::from("Circumcised"), String::from("Uncircumcised")]
    }
}

impl FromString for Circumcision {
    fn from_string(circumcision: &str) -> Circumcision {
        match circumcision {
            "Circumcised" => Circumcision::Circumcised,
            "Uncircumcised" => Circumcision::Uncircumcised,
            _ => panic!("Invalid circumcision"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circumcision_score() {
        let circumsized = Circumcision::Circumcised;
        let uncircumsized = Circumcision::Uncircumcised;

        assert_eq!(circumsized, Circumcision::Circumcised);
        assert_eq!(uncircumsized, Circumcision::Uncircumcised);
    }
}
