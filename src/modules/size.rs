use crate::{GetVariants, Score, FromString};

/// Struct representing a size in centimeters (cm), composed of length and girth.
#[derive(Debug, PartialEq)]
pub struct SizeCM {
    pub length: f32, // in cm
    pub girth: f32,  // in cm
}

/// Implementation of Score trait for SizeCM. The score is calculated based on both length and girth.
impl Score for SizeCM {
    fn score(&self) -> u32 {
        // Calculate scores for length and girth
        // Scores are determined by the ranges the length and girth fall into
        // Note: the lengths and girths are first multiplied by 10.0 to facilitate range comparison
        // The final score is the sum of the length score and the girth score.

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

/// Struct representing a size in inches (in), composed of length and girth.
#[derive(Debug, PartialEq)]
pub struct SizeIN {
    pub length: f32, // in in
    pub girth: f32,  // in in
}

/// Implementation of Score trait for SizeIN. Similar to SizeCM, the score is calculated based on both length and girth.
impl Score for SizeIN {
    fn score(&self) -> u32 {
        let length_score = match (self.length * 10.0) as u32 {
            0..=35 => 1,
            36..=50 => 2,
            51..=57 => 3,
            58..=70 => 4,
            71..=80 => 5,
            _ => 2,
        };

        let girth_score = match (self.girth * 10.0) as u32 {
            0..=33 => 1,
            34..=40 => 2,
            41..=46 => 3,
            47..=54 => 4,
            55..=63 => 5,
            _ => 2,
        };

        length_score + girth_score
    }
}

/// [SizeType] is an enum that represents the type of size.
#[derive(Debug, PartialEq)]
pub enum SizeType {
    Centimeters,
    Inches,
}

/// The [Score] trait implementation for [SizeType] provides a score value based on the size type.
impl GetVariants for SizeType {
    fn get_variants() -> Vec<String> {
        vec![String::from("Centimeters"), String::from("Inches")]
    }
}

/// The [FromString] trait implementation for [SizeType] returns a [SizeType] variant based on the string provided.
impl FromString for SizeType {
    fn from_string(size_type: &str) -> SizeType {
        match size_type {
            "Centimeters" => SizeType::Centimeters,
            "Inches" => SizeType::Inches,
            _ => panic!("Invalid size type"),
        }
    }
}

/// [Size] is a struct that represents a size.
#[derive(Debug)]
pub struct Size {
    pub length: f32,
    pub girth: f32,
    pub size_type: SizeType,
}

/// Implementation of Score trait for Size. The score is calculated based on the size type.
impl Size {
    /// Function to create a Size instance from given length and girth in centimeters.
    pub fn from_cm(length: f32, girth: f32) -> Size {
        Size {
            length,
            girth,
            size_type: SizeType::Centimeters,
        }
    }

    /// Function to create a Size instance from given length and girth in inches.
    pub fn from_in(length: f32, girth: f32) -> Size {
        Size {
            length,
            girth,
            size_type: SizeType::Inches,
        }
    }

    /// Function to calculate the score of a Size instance. 
    pub fn score(&self) -> u32 {
        match self.size_type {
            SizeType::Centimeters => SizeCM {
                length: self.length,
                girth: self.girth,
            }
            .score(),
            SizeType::Inches => SizeIN {
                length: self.length,
                girth: self.girth,
            }
            .score(),
        }
    }

    /// Function to get the size as a string, with the appropriate unit (cm/in) depending on the size type.
    pub fn get_size(&self) -> String {
        match self.size_type {
            SizeType::Centimeters => format!("{}cm x {}cm", self.length, self.girth),
            SizeType::Inches => format!("{}in x {}in", self.length, self.girth),
        }
    }

}

/// Implementation of Display trait for Size. Similar to get_size, but used for formatting.
impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.size_type {
            SizeType::Centimeters => write!(f, "{}cm x {}cm", self.length, self.girth),
            SizeType::Inches => write!(f, "{}in x {}in", self.length, self.girth),
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
