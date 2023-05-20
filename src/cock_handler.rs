use crate::{
    CockStruct,
    Tier,
    Score,
    ID
};

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
        let total_possible_score: u32 = 60;
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
            83..=93 => Tier::A,
            76..=82 => Tier::B,
            61..=75 => Tier::C,
            51..=60 => Tier::D,
            41..=50 => Tier::E,
            _ => Tier::F,
        }
    }
}

// add testing
#[cfg(test)]
mod tests {
}
