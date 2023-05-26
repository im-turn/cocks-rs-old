//! A library for comprehensive grading of cocks.
//!
//! `cock-tier` provides a modular, extensible, and detailed system for
//! grading and classifying cocks based on various metrics.
//!
//! This library allows users to:
//! - Assign scores based on multiple factors such as size, shape, and aesthetics.
//! - Easily extend the functionality by adding new grading methods.
//! - Obtain a comprehensive summary of the cocks score, grade, stats, etc.
//!
//! # Example
//! ```
//! use cock_tier::{CockStruct, Size, Aesthetic, Balls, Shape, Curvature, Circumcision, Veininess, Abnormalities, Inches};
//!
//! let cock = CockStruct::new(
//!     Size {
//!         length: 5.5,
//!         girth: 4.5,
//!         size_type: Inches,
//!     },
//!     Aesthetic::Normal,
//!     Balls::Normal,
//!     Shape::Cylindrical,
//!     Curvature::Straight,
//!     Circumcision::Uncircumcised,
//!     Veininess::Normal,
//!     Abnormalities::None,
//! );
//!
//! // Perform your operations on `cock`
//! ```
//!
//! # TODO
//! add more examples

mod modules;

#[allow(unused_attributes, unused_imports)]
pub use modules::{
    abnormalities::Abnormalities,
    aesthetic::Aesthetic,
    balls::Balls,
    bin_modules,
    circumcision::Circumcision,
    cock_handler::{CockHandler, CockResult},
    cock_struct::CockStruct,
    curvature::Curvature,
    shape::Shape,
    size::{
        Size,
        SizeType::{self, Centimeters, Inches},
    },
    tier::Tier,
    traits::{FromString, GetVariants, Score},
    user::{User as InnerUser, ID},
    veininess::Veininess,
};
