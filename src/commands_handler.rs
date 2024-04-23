use rand::prelude::{thread_rng, SliceRandom};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use text_io::read;

use crate::animal_alive_csv::AnimalAliveReader;
use crate::animal_data_csv::AnimalDataReader;
use crate::animal_structs::*;
use crate::clear_console::clear_terminal_screen;
use crate::common_paths::*;
use crate::custom_writer::CustomWriter;
use crate::file_handler::*;

pub fn handle_input(command_name: &str, animal_name: &str) -> Result<(), Box<dyn Error>> {
    match command_name {
        "create" => create_animal(animal_name),
        "delete" => delete_animal(animal_name),
        "read" => read_animal(animal_name),
        "spawn" => spawn_animal(animal_name),
        "reproduce" => reproduce_animal(animal_name),
        "kill" => kill_animal(animal_name),
        "feed" => feed_animal(animal_name),
        "count" => count_animal(animal_name),
        _ => Err("Invalid command".into()),
    }
}

fn get_valid_user_input(label: &str, validator: &dyn Fn(&String) -> bool) -> String {
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

fn create_animal(animal_name: &str) -> Result<(), Box<dyn Error>> {
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

    let animals_file_write = animals_file_write_append(ANIMALS_DATA_FILE_PATH)?;
    let mut writer = CustomWriter::new(csv::Writer::from_writer(animals_file_write));

    writer.write_animal_data_file(animal)?;

    Ok(())
}

fn valid_class(input: &String) -> bool {
    [
        "reptile",
        "mammal",
        "fish",
        "bird",
        "arthropod",
        "amphibian",
    ]
    .contains(&input.as_str())
}
fn valid_predators(input: &String) -> bool {
    valid_string(input)
}
fn valid_preys(input: &String) -> bool {
    valid_string(input)
}

fn valid_string(input: &String) -> bool {
    let pattern = Regex::new(r"^[a-z]+(?:-[a-z]+)*$").unwrap();
    pattern.is_match(input)
}

trait CustomStringMethods {
    fn to_class(&self) -> Result<Class, Box<dyn Error>>;
}

impl CustomStringMethods for String {
    fn to_class(&self) -> Result<Class, Box<dyn Error>> {
        match self.as_str() {
            "reptile" => Ok(Class::Reptile),
            "mammal" => Ok(Class::Mammal),
            "fish" => Ok(Class::Fish),
            "bird" => Ok(Class::Bird),
            "arthropod" => Ok(Class::Arthropod),
            "amphibian" => Ok(Class::Amphibian),
            _ => return Err("Invalid class".into()),
        }
    }
}

pub fn delete_animal(animal_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
fn read_animal(animal_name: &str) -> Result<(), Box<dyn Error>> {
    let animals_file_read = File::open(ANIMALS_DATA_FILE_PATH)?;
    let mut reader = csv::Reader::from_reader(animals_file_read);

    if reader.animal_data_in_file(animal_name)? {
        let animal_data = reader.read_animal_data(animal_name)?.unwrap();
        println!(
            "Name: {}\nClass: {}\nPredators: {}\nPreys: {}",
            animal_data.name,
            animal_data.class_str(),
            animal_data.predators_str(),
            animal_data.preys_str()
        );
    } else {
        println!("Animal not found");
    }

    Ok(())
}
fn spawn_animal(animal_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
fn reproduce_animal(animal_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
fn kill_animal(animal_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
fn feed_animal(animal_name: &str) -> Result<(), Box<dyn Error>> {
    if animal_data_in_file(animal_name)? {
        feed_animal(animal_name);
        let animal_data = reader.read_animal_data(animal_name)?.unwrap();

        let mut rng = thread_rng();
        let prey = animal_data.preys().choose(&mut rng).unwrap();

        println!("Mmm... That {} was delicious", prey);
    } else {
        println!("Animal not found");
    }

    Ok(())
}
fn count_animal(animal_name: &str) -> Result<(), Box<dyn Error>> {
    let count = count_animal(animal_name)?;
    println!("We have {} {}s!", count, animal_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
