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

// add testing
#[cfg(test)]
mod tests {
}
