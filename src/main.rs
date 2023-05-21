use cock_tier::{
    BIN::{
        get_user,
        get_size,
        get_enum_variant
    },
    Abnormalities, Aesthetic, Balls, Circumcision, CockHandler,
    CockStruct, Curvature, Shape,
    Veininess
};

fn main() {
    let user = get_user();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let size = get_size();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let aes_choice = get_enum_variant::<Aesthetic>("Choose aesthetic: ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let ball_choice = get_enum_variant::<Balls>("Choose balls: ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let shape_choice = get_enum_variant::<Shape>("Choose shape: ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let curv_choice = get_enum_variant::<Curvature>("Choose curvature: ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let circ_choice = get_enum_variant::<Circumcision>("Choose circumcision: ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let vein_choice = get_enum_variant::<Veininess>("Choose veininess: ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let abnor_choice = get_enum_variant::<Abnormalities>("Choose abnormalities: ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let cock = CockStruct {
        size,
        aesthetic: aes_choice,
        balls: ball_choice,
        shape: shape_choice,
        curvature: curv_choice,
        circumcision: circ_choice,
        veininess: vein_choice,
        abnormalities: abnor_choice
    };

    println!("{:#?}\n", cock);

    let handler = CockHandler {
        id: user,
        cock
    };

    println!();

    println!("{:?}\n{:?}\n{}\n{}", handler.id, handler.grade(), handler.total_score().score, handler.total_score().percentage)
}

