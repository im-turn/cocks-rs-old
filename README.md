# Turn's Cock Tier Evaluator

The Cock Tier Evaluator is a Rust-based library for categorizing and scoring features of the male genitalia. 

Scoring and judgements are based on the input from the almight 'turn' himself.

## Modules
- **tier:** Provides a classification system for the overall quality of a penis, based on its aggregate score.
- **traits:** Contains the Rust traits used throughout the library, currently just includes the 'Score' trait.
- **size:** Contains functionality for measuring penis size in either inches or centimeters.
- **aesthetic:** Provides a system for scoring the aesthetic appeal of a penis.
- **balls:** Provides a system for scoring the size and shape of the testicles.
- **shape:** Provides a system for categorizing and scoring the overall shape of a penis.
- **curvature:** Provides a system for categorizing the curvature of a penis.
- **circumcision:** Contains functionality for representing the circumcision status of a penis.
- **veininess:** Provides a system for categorizing and scoring the veininess of a penis.
- **abnormalities:** Contains functionality for representing and scoring any abnormalities in a penis's appearance.
- **cock_struct:** A composite struct that combines all other aspects of a penis into a single entity.
- **user:** Contains functionality for representing and managing the user who owns a penis.
- **cock_handler:** This module wraps up the `User` and `CockStruct` to provide easy methods to calculate and print details about the penile structure.

## Usage

1. Create a `CockStruct` instance using the `CockStruct::new` method. You will need to provide all necessary traits as arguments.
2. Instantiate a `CockHandler` object, using the `User` and `CockStruct` as arguments.
3. Call the `grade` method to determine the overall quality tier of the penis.
4. Use the `total_score` method to get the total score of all the penis's traits, as well as a percentage-based overall quality score.

## Example

```rust
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
```
&#8595;&#8595; OUTPUT &#8595;&#8595;
```sh
CockHandler {
    id: Anonymous,
    cock: CockStruct {
        size: Size {
            length: 5.5,
            girth: 4.5,
            size_type: Inches,
        },
        aesthetic: Normal,
        balls: Normal,
        shape: Cylindrical,
        curvature: Straight,
        circumcision: Uncircumcised,
        veininess: Normal,
        abnormalities: None,
    },
}
Grade: C
Score: 38
Percentage: 63.333332
```

## Testing

This library comes with a built-in test suite that verifies the functionality of the code. To run the tests, use the command `cargo test` in the project's root directory.

## License

MIT License
