use cock_tier::{
    BIN::cock_handler_build,
};

fn main() {
    let handler = cock_handler_build();

    println!("{}", handler)
}
