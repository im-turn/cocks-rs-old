#[derive(Debug, PartialEq)]
pub enum Circumcision{
    Circumcised,
    Uncircumcised,
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