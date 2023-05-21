pub trait Score {
    fn score(&self) -> u32;
}

pub trait GetVariants {
    fn get_variants() -> Vec<String>;
}

pub trait FromString {
    fn from_string(string: &str) -> Self;
}