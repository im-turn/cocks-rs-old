pub fn prompt(msg: &str) -> String {
    println!("{}\n", msg);
    let mut input = String::new();
    print!(">> ");

    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub fn parse_to_int(input: &str, default: i32) -> i32 {
    match input.parse::<i32>() {
        Ok(n) => n,
        Err(_) => default,
    }
}

pub fn parse_to_float(input: &str, default: f32) -> f32 {
    match input.parse::<f32>() {
        Ok(n) => n,
        Err(_) => default,
    }
}

pub fn confirm(message: &str) -> bool {
    loop {
        println!("[Y/N]");
        let response = prompt(message);
        match response.to_uppercase().as_str() {
            "Y" => return true,
            "N" => return false,
            _ => println!("Invalid response."),
        }
    }
}

pub fn choose_from_menu(choices: Vec<String>, msg: &str) -> String {
    println!("Please choose an option:\n");

    let choice_len = choices.len()
                            .try_into()
                            .expect("Too many choices.");

    for (i, choice) in choices.iter().enumerate() {
        println!("{}. {}", i + 1, choice);
    }

    println!();

    let selection = loop {
        let choice = prompt(msg);
        let choice = parse_to_int(&choice, 0);

        if choice > 0 && choice <= choice_len {
            break (choice - 1) as usize;
        } else {
            println!("Invalid choice.");
        }
    };

    choices.get(selection)
        .unwrap()
        .to_string()
}


use crate::{GetVariants, FromString, ID, User as InnerUser, Size, SizeType};

pub fn get_enum_variant<T: GetVariants + FromString>(message: &str) -> T {
    let choice = choose_from_menu(T::get_variants(), message);
    T::from_string(&choice)
}

pub fn get_user() -> ID {
    let user_type = choose_from_menu(ID::get_variants(), "ID type:");
    match user_type.as_str() {
        "Anonymous" => ID::Anonymous,
        "User" => {
            let name = prompt("User Name:");
            let discord_name = prompt("Discord Name:");
            ID::User(
                InnerUser {
                    name,
                    discord_name
                }
            )
        },
        _ => panic!("Invalid user type")
    }
}

pub fn get_size() -> Size {
    let size_type = choose_from_menu(SizeType::get_variants(), "Inches or Centimeters?");
    let length = parse_to_float(prompt("Cock length:").as_str(), 0.0);
    let girth = parse_to_float(prompt("Cock girth:").as_str(), 0.0);

    match size_type.as_str() {
        "Inches" => Size::from_in(length, girth),
        "Centimeters" => Size::from_cm(length, girth),
        _ => panic!("Invalid size type")
    }
}
