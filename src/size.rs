use crate::Score;

#[derive(Debug)]
pub struct SizeCM {
    pub length: f32, // in cm
    pub girth: f32, // in cm
}

impl Score for SizeCM {
    fn score(&self) -> u32 {
        let length_score = match (self.length * 10.0) as u32 {
            0..=90 => 1,
            91..=125 => 2,
            126..=145 => 3,
            146..=175 => 4,
            176..=200 => 5,
            _ => 1,
        };

        let girth_score = match (self.girth * 10.0) as u32 {
            0..=85 => 1,
            86..=100 => 2,
            101..=115 => 3,
            116..=135 => 4,
            136..=155 => 5,
            _ => 1,
        };

        length_score + girth_score
    }

}

#[derive(Debug)]
pub struct SizeIN {
    pub length: f32, // in in
    pub girth: f32, // in in
}

impl Score for SizeIN {
    fn score(&self) -> u32 {
        let length_score = match (self.length * 10.0) as u32 {
            0..=35 => 1,
            36..=50 => 2,
            51..=57 => 3,
            58..=70 => 4,
            71..=80 => 5,
            _ => 1,
        };

        let girth_score = match (self.girth * 10.0) as u32 {
            0..=33 => 1,
            34..=40 => 2,
            41..=46 => 3,
            47..=54 => 4,
            55..=63 => 5,
            _ => 1,
        };

        length_score + girth_score
    }
}

#[derive(Debug)]
pub enum SizeType {
    Centimeters,
    Inches,
}

#[derive(Debug)]
pub struct Size {
    pub length: f32,
    pub girth: f32,
    pub size_type: SizeType,
}

impl Size {
    pub fn from_cm(length: f32, girth: f32) -> Size {
        Size {
            length,
            girth,
            size_type: SizeType::Centimeters,
        }
    }

    pub fn from_in(length: f32, girth: f32) -> Size {
        Size {
            length,
            girth,
            size_type: SizeType::Inches,
        }
    }

    pub fn score(&self) -> u32 {
        match self.size_type {
            SizeType::Centimeters => SizeCM {
                length: self.length,
                girth: self.girth,
            }.score(),
            SizeType::Inches => SizeIN {
                length: self.length,
                girth: self.girth,
            }.score(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_score() {
        let size_cm = Size::from_cm(14.0, 10.0);
        let size_in = Size::from_in(5.5, 4.0);

        assert_eq!(size_cm.score(), 5);
        assert_eq!(size_in.score(), 5);
    }

    #[test]
    fn test_size_cm_score() {
        let size = SizeCM {
            length: 14.0,
            girth: 10.0,
        };

        assert_eq!(size.score(), 5);
    }

    #[test]
    fn test_size_in_score() {
        let size = SizeIN {
            length: 5.5,
            girth: 4.0,
        };

        assert_eq!(size.score(), 5);
    }
}
