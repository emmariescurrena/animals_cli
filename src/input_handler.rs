use std::error::Error;
use text_io::read;

use crate::animal_structs::{AnimalData, Sex};
use crate::clear_console::clear_terminal_screen;
use crate::commands::*;
use crate::custom_string_methods::*;
use crate::input_validators::*;

pub fn handle_inputs() -> Result<(), Box<dyn Error>> {
    loop {
        println!("Insert a command, type help to see commands available or exit to... exit");
        let command: String = read!();
        clear_terminal_screen();
        let command_output = match command.as_str() {
            "create" => create_animal(get_inputs_create_animal()?)?,
            "delete" => {
                let animal_name = get_animal_name_from_input()?;
                if confirm_deletion(&animal_name) {
                    delete_animal(animal_name)?
                } else {
                    "Deletion aborted".to_string()
                }
            }
            "read" => read_animal(get_animal_name_from_input()?)?,
            "spawn" => spawn_animal(get_animal_name_from_input()?, get_animal_sex_from_input()?)?,
            "reproduce" => reproduce_animal(get_animal_name_from_input()?)?,
            "kill" => kill_animal(get_animal_name_from_input()?)?,
            "feed" => feed_animal(get_animal_name_from_input()?)?,
            "count" => count_animal(get_animal_name_from_input()?)?,
            "help" => help(),
            "exit" => break,
            _ => "Invalid command".to_string(),
        };
        println!("{}", command_output);
    }
    Ok(())
}

pub fn get_valid_user_input(label: &str, validator: &dyn Fn(&String) -> bool) -> String {
    loop {
        println!("{}", label);
        let input = read!();
        if validator(&input) {
            clear_terminal_screen();
            return input;
        }
        clear_terminal_screen();
        println!("Invalid input");
    }
}

fn get_animal_name_from_input() -> Result<String, Box<dyn Error>> {
    Ok(get_valid_user_input(
        "Insert animal name, like snake or polar-bear",
        &valid_animal_name,
    ))
}

fn get_inputs_create_animal() -> Result<AnimalData, Box<dyn Error>> {
    let animal_name = get_valid_user_input(
        "Insert animal name, like snake or polar-bear",
        &valid_animal_name,
    );
    let animal_class = get_valid_user_input(
        "Insert animal class (reptile, mammal, fish, bird, arthropod or amphibian)",
        &valid_class,
    );
    let animal_predators = get_valid_user_input(
        "Insert animal predators, separated by a '-'. Example: 'cow-snake-shark'",
        &valid_predators,
    );
    let animal_preys = get_valid_user_input(
        "Insert animal preys, separated by a '-'. Example: 'cow-snake-shark'",
        &valid_preys,
    );
    let animal = AnimalData::new(
        animal_name,
        animal_class.to_class()?,
        animal_predators.split("-").map(|s| s.to_string()).collect(),
        animal_preys.split("-").map(|s| s.to_string()).collect(),
    );
    Ok(animal)
}

fn confirm_deletion(animal_name: &String) -> bool {
    let label = format!(
        "Are you sure to delete {} data? Insert 'y' to confirm or any other character to return",
        animal_name
    );
    if get_valid_user_input(label.as_str(), &any_input_is_valid) == "y" {
        true
    } else {
        false
    }
}

fn get_animal_sex_from_input() -> Result<Sex, Box<dyn Error>> {
    match get_valid_user_input("Type 'm' to born a male or 'f' for a female", &valid_sex).as_str() {
        "m" => Ok(Sex::Male),
        "f" => Ok(Sex::Female),
        _ => Err("Invalid sex".into()),
    }
}
