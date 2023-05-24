pub mod abnormalities;
pub mod aesthetic;
pub mod balls;
pub mod circumcision;
pub mod cock_handler;
pub mod cock_struct;
pub mod curvature;
pub mod shape;
pub mod size;
pub mod tier;
pub mod traits;
pub mod user;
pub mod veininess;
pub mod bin_modules;

pub use self::{
    bin_modules::{
        standard_prompt,
        tui_prompt
    },
    abnormalities::Abnormalities,
    aesthetic::Aesthetic,
    balls::Balls,
    circumcision::Circumcision,
    cock_handler::CockHandler,
    cock_struct::CockStruct,
    curvature::Curvature,
    shape::Shape,
    size::{
        Size, SizeCM, SizeIN,
        SizeType::{Centimeters, Inches},
    },
    tier::Tier,
    traits::{GetVariants, Score, FromString},
    user::{User, ID},
    veininess::Veininess,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let user = ID::Anonymous;

        let cock = CockStruct::new(
            Size {
                length: 5.5,
                girth: 4.5,
                size_type: Inches,
            },
            Aesthetic::Normal,
            Balls::Normal,
            Shape::Cylindrical,
            Curvature::Straight,
            Circumcision::Uncircumcised,
            Veininess::Normal,
            Abnormalities::None,
        );

        let handler = CockHandler { id: user, cock };

        assert_eq!(handler.grade(), Tier::C);
        assert_eq!(handler.total_score().score, 48.0);
        assert_eq!(handler.total_score().percentage, 68.571434);
    }
}
