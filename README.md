# PLEASE CHECK OUT THE NEW REPOSITORY [HERE](https://github.com/im-turn/cock-lib). THIS CRATE WILL NO LONGER BE MAINTAINED.


# Turn's Cock Tier Evaluator :chicken: :trophy:

Welcome to Turn's Cock Tier Evaluator, a Rust-based command-line application where your one-eyed monster gets the recognition it deserves!

The ratings are based on cock rubrics created by turn himself! 

## The Anatomy of the Evaluator :eggplant: :stethoscope:

I've dissected the cock-rating task into neat, testable modules. Here's a quick breakdown:

- **bin_modules (bin_modules.rs):** Our little helpers, the `bin_modules` are exclusively used within the operations of the executables and won't be found outside of this module and the `bin` files. It's where things such as the `cock_handler_build` function is located, which creates a `CockHandler` based on your personal input during the execution of the default binary `cock-tier`/`main.rs`.
- **User (user.rs):** You, the user, the cock-owner.
- **CockStruct (cock_struct.rs):** Your magnificent (or not so magnificient) monument's blueprint, combining all its noteworthy aspects into a single entity.
- **CockHandler (cock_handler.rs):** The gentle hands wrapping up the `User` and `CockStruct` to provide easy methods to calculate and print all the juicy details about your member.
- **Traits (traits.rs):** These aren't your cock traits, they're various Rust traits used throughout the application.
- **Various feature modules:** Each of these modules, like `size.rs`, `aesthetic.rs`, `balls.rs` (and others), represents a particular feature of your shlong, providing a score for several of them (a few features have no impact on ratings as of right now).

## Getting Rated and Library Usage :open_book: :male_detective:

Want to see where you stand in the land of peen, but you're kinda a dummy?! Luckily I'm here to guide you through the process.

### The Executable Walkthrough

To use the default cli based binary, just run `cargo run` or `cargo run --bin cock-tier` from your terminal and follow the prompts. It's very straight-forward tbh. The program will ask you a series of questions about the cock in question, once the questions have run their course you get a comprehensive breakdown complete with a final grade.

Other binaries available include:

- tui: a terminal user interface that operates similarly to the default cli binary. The user interface is produced using the `cursive` rust crate which acts as a ncurses wrapper for rust. To run this binary use the command `cargo run --bin tui`. Still a work in progress, but should be somewhat functional. Below is a small demonstation

![tui-demo](/assets/tui-demo.gif)


### The API

You're a developer who sees the inherent value in this? Strange. However, it is fairly simple to get started with things. To begin, add the following to your `Cargo.toml` file:

```toml
[dependencies]
cock-tier = "x.x.x"
```

Below is an example of one way you could go about using API.

```rust
use cock_tier::{
    Abnormalities, Aesthetic, Balls, Circumcision, CockHandler, CockStruct, Curvature,
    Shape, Size, SizeType::Inches, Tier, ID, Veininess,
};
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
    Abnormalities::None,
);
let handler = CockHandler { id: user, cock };
// Now you can access the CockHandler's methods:
let grade = handler.grade(); // Returns Tier::C
let score = handler.total_score().score; // Returns 48.0
let percentage = handler.total_score().percentage; // Returns 68.571434
```

## Dependencies

The `tui` executable for cock-tier is rendered using [Cursive](https://crates.io/crates/cursive).

[Cursive](https://crates.io/crates/cursive) also requires the `ncurses` lib to be installed on the machine.

## Testing :petri_dish: :test_tube:

This library comes with a built-in test suite that verifies the functionality of the code. To run the tests, use the command `cargo test` in the project's root directory.

## TODO

- finish documentation
- increase testing coverage
- additional binaries for multiple methods of operation
- increased cock analysis capabilities
- etc

## License :clipboard: :briefcase:

MIT License