/// [Score] is a trait for objects that can be scored.
/// The score function returns a [u32] representing the score of the object.
pub trait Score {
    fn score(&self) -> u32;
}

/// [GetVariants] is a trait for objects that have a set of variants.
/// The [GetVariants::get_variants] function returns a vector of strings, each representing a variant of the object.
pub trait GetVariants {
    fn get_variants() -> Vec<String>;
}

/// [FromString] is a trait for objects that can be constructed from a string.
/// The [FromString::from_string] function takes a string reference as input and returns an instance of the type implementing this trait based on the string.
pub trait FromString {
    fn from_string(string: &str) -> Self;
}

/// [TUIDisplay] is a trait for drawing screens within the TUI.
/// The [TUIDisplay::draw] function takes a mutable reference to a [Cursive] instance as input and draws the screen.
pub trait TUIDisplay {
    fn draw(&self, s: &mut cursive::Cursive);
}