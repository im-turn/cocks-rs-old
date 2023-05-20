use crate::{
    CockStruct,
    Tier,
    Score,
    ID
};

#[derive(Debug)]
pub struct CockResult {
    pub score: f32,
    pub percentage: f32,
}

#[derive(Debug)]
pub struct CockHandler {
    pub id: ID,
    pub cock: CockStruct,
}

impl CockHandler {
    pub fn new(id: ID, cock: CockStruct) -> CockHandler {
        CockHandler { id, cock }
    }

    pub fn total_score(&self) -> CockResult {
        let total_possible_score: u32 = 46;
        let actual_score = (self.cock.aesthetic.score() + self.cock.balls.score() + self.cock.veininess.score() + self.cock.abnormalities.score() + self.cock.size.score()) as f32 * 2.0;
        let percentage_score = actual_score / total_possible_score as f32 * 100.0;

        CockResult {
            score: actual_score,
            percentage: percentage_score
        }
    }

    pub fn grade(&self) -> Tier {
        let percentage_score = self.total_score().percentage;
        
        let score = percentage_score as i32;

        match score {
            94..=100 => Tier::S,
            86..=93 => Tier::A,
            77..=84 => Tier::B,
            68..=76 => Tier::C,
            58..=67 => Tier::D,
            47..=57 => Tier::E,
            _ => Tier::F,
        }
    }
}

#[cfg(test)]
use crate::{
    User,
    Size,
    Inches,
    Aesthetic,
    Balls,
    Shape,
    Curvature,
    Circumcision,
    Veininess,
    Abnormalities
};

#[test]
fn cock_handler_test() {
    let user = ID::User(
        User {
            name: String::from("turn"),
            discord_name: String::from("turn")
        }
    );

    let cock = CockStruct::new(
        Size {
            length: 6.7,
            girth: 4.5,
            size_type: Inches,
        },
        Aesthetic::PrettyNice,
        Balls::Normal,
        Shape::Cylindrical,
        Curvature::Straight,
        Circumcision::Circumcised,
        Veininess::Normal,
        Abnormalities::None
    );

    let handler = CockHandler {
        id: user,
        cock,
    };

    println!(
        "\n{:#?}\nGrade: {:?}\nScore: {}\nPercentage: {:?}",
        handler,
        handler.grade(),
        handler.total_score().score,
        handler.total_score().percentage
    );
}

#[test]
fn cock_handler_test2() {
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

    println!(
        "\n{:#?}\nGrade: {:?}\nScore: {}\nPercentage: {:?}",
        handler,
        handler.grade(),
        handler.total_score().score,
        handler.total_score().percentage
    );
}
