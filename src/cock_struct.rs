use crate::{
    Size,
    Aesthetic,
    Balls,
    Shape,
    Curvature,
    Circumcision,
    Veininess,
    Abnormalities,
};

#[derive(Debug)]
pub struct CockStruct {
    pub size: Size,
    pub aesthetic: Aesthetic,
    pub balls: Balls,
    pub shape: Shape,
    pub curvature: Curvature,
    pub circumcision: Circumcision,
    pub veininess: Veininess,
    pub abnormalities: Abnormalities,
}

impl CockStruct {
    pub fn new(
        size: Size,
        aesthetic: Aesthetic,
        balls: Balls,
        shape: Shape,
        curvature: Curvature,
        circumcision: Circumcision,
        veininess: Veininess,
        abnormalities: Abnormalities,
    ) -> CockStruct {
        CockStruct {
            size,
            aesthetic,
            balls,
            shape,
            curvature,
            circumcision,
            veininess,
            abnormalities,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Inches,
    };

    #[test]
    fn cock_struct_test() {
        let cock = CockStruct::new(
            Size {
                length: 5.6,
                girth: 4.1,
                size_type: Inches,
            },
            Aesthetic::UglyButUsable,
            Balls::PossibleCancer,
            Shape::Other(
                String::from("test")
            ),
            Curvature::Right,
            Circumcision::Circumcised,
            Veininess::HealthyPumper,
            Abnormalities::Major(
                String::from("Active case of herpes")
            )
        );

        assert_eq!(cock.size.length, 5.6);
        assert_eq!(cock.size.girth, 4.1);
        assert_eq!(cock.size.size_type, Inches);
        assert_eq!(cock.aesthetic, Aesthetic::UglyButUsable);
        assert_eq!(cock.balls, Balls::PossibleCancer);
        assert_eq!(cock.shape.get_shape(), "test");
        assert_eq!(cock.curvature, Curvature::Right);
        assert_eq!(cock.circumcision, Circumcision::Circumcised);
        assert_eq!(cock.veininess, Veininess::HealthyPumper);
        assert_eq!(cock.abnormalities.get_abnormality(), "Active case of herpes");
    }
}
