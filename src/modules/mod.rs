pub mod tier;
pub mod traits;
pub mod size;
pub mod aesthetic;
pub mod balls;
pub mod shape;
pub mod curvature;
pub mod circumcision;
pub mod veininess;
pub mod abnormalities;
pub mod cock_struct;
pub mod user;
pub mod cock_handler;

pub use self::{
    tier::Tier,
    traits::Score,
    size::{
        Size,
        SizeCM,
        SizeIN,
        SizeType::{
            Centimeters,
            Inches,
        },
    },
    aesthetic::Aesthetic,
    balls::Balls,
    shape::Shape,
    curvature::Curvature,
    circumcision::Circumcision,
    veininess::Veininess,
    abnormalities::Abnormalities,
    cock_struct::CockStruct,
    user::{
        ID,
        User
    },
    cock_handler::CockHandler
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
            Abnormalities::None
        );

        let handler = CockHandler {
            id: user,
            cock,
        };

        assert_eq!(handler.grade(), Tier::C);
        assert_eq!(handler.total_score().score, 38.0);
        assert_eq!(handler.total_score().percentage, 63.333332);
    }
}
