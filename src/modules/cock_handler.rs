use crate::{CockStruct, Score, Tier, ID};

#[derive(Debug, PartialEq)]
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
        let actual_score = (
            self.cock.aesthetic.score()         // 10
            + self.cock.balls.score()           // 5
            + self.cock.veininess.score()       // 5
            + self.cock.abnormalities.score()   // 5
            + self.cock.size.score()) as f32    // 10
            * 2.0;

        let total_possible_score = 70.0;

        let percentage_score = actual_score / total_possible_score * 100.0;

        CockResult {
            score: actual_score,
            percentage: percentage_score,
        }
    }

    pub fn grade(&self) -> Tier {
        let percentage_score = self.total_score().percentage;

        let score = percentage_score as i32;

        match score {
            91..=100 => Tier::S,
            81..=90 => Tier::A,
            71..=80 => Tier::B,
            61..=70 => Tier::C,
            51..=60 => Tier::D,
            41..=50 => Tier::E,
            _ => Tier::F,
        }
    }
}

// add testing
#[cfg(test)]
mod tests {}
