use rand::prelude::{thread_rng, SliceRandom};
use std::error::Error;

use crate::animal_alive_csv::{
    kill_all_animals_alive, kill_one_animal_alive, reader_animals_alive, writer_animals_alive,
    AnimalAliveReader,
};
use crate::animal_data_csv::{
    delete_animal_data, reader_animals_data, writer_animals_data, AnimalDataReader,
};
use crate::animal_structs::*;

pub fn animal_data_registered(animal_name: &str) -> Result<bool, Box<dyn Error>> {
    let mut reader = reader_animals_data()?;
    if reader.read_animal_data(animal_name)?.is_some() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn animal_alive(animal_name: &str) -> Result<bool, Box<dyn Error>> {
    let mut reader = reader_animals_alive()?;
    if reader.read_animal_alive(animal_name)?.is_some() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn create_animal(animal: AnimalData) -> Result<String, Box<dyn Error>> {
    let animal_name = animal.name();
    if animal_data_registered(&animal_name)? {
        Ok(format!("{} already exists", animal_name))
    } else {
        let mut writer = writer_animals_data()?;
        writer.write_animal_data(&animal)?;
        Ok(format!("{} data added", animal_name))
    }
}

pub fn delete_animal(animal_name: String) -> Result<String, Box<dyn Error>> {
    let animal_name = animal_name.as_str();
    delete_animal_data(animal_name)?;
    kill_all_animals_alive(animal_name)?;
    Ok(format!("{} is now extinct!", animal_name))
}

pub fn read_animal(animal_name: String) -> Result<String, Box<dyn Error>> {
    let mut reader = reader_animals_data()?;

    if let Some(animal_data) = reader.read_animal_data(animal_name.as_str())? {
        Ok(format!(
            "Name: {}\nClass: {}\nPredators: {}\nPreys: {}",
            animal_data.name,
            animal_data.class_str(),
            animal_data.predators_str(),
            animal_data.preys_str()
        ))
    } else {
        Ok("Animal not found".to_string())
    }
}
pub fn spawn_animal(animal_name: String, animal_sex: Sex) -> Result<String, Box<dyn Error>> {
    if !animal_data_registered(&animal_name)? {
        return Ok(format!("{} does not exist", animal_name));
    }
    let mut writer = writer_animals_alive()?;

    let animal_name = animal_name.as_str();
    let animal = AnimalAlive::born(animal_name, animal_sex);
    let animal_sex = animal.sex_str();
    writer.write_animal_alive(animal)?;
    Ok(format!(
        "A {} has borned! It's a {}",
        animal_name, animal_sex
    ))
}

pub fn reproduce_animal(animal_name: String) -> Result<String, Box<dyn Error>> {
    let mut reader = reader_animals_alive()?;
    if !animal_alive(&animal_name)? {
        return Ok(format!("{} is not even alive", animal_name));
    } else if reader.has_both_sexes(animal_name.as_str())? {
        let animal_sex = {
            let mut rng = thread_rng();
            [Sex::Male, Sex::Female].choose(&mut rng).unwrap().clone()
        };
        Ok(spawn_animal(animal_name, animal_sex)?)
    } else {
        Ok(format!("{} does not have both sexes", animal_name))
    }
}

pub fn kill_animal(animal_name: String) -> Result<String, Box<dyn Error>> {
    let animal_name = animal_name.as_str();
    let mut reader = reader_animals_data()?;
    if !animal_alive(&animal_name)? {
        Ok(format!("{} is not even alive", animal_name))
    } else if let Some(animal_data) = reader.read_animal_data(animal_name)? {
        let mut rng = thread_rng();
        let predator = animal_data.predators().choose(&mut rng).unwrap();
        kill_one_animal_alive(animal_name)?;

        Ok(format!("{} was killed by a {}", animal_name, predator))
    } else {
        Ok(format!("{} does not exist", animal_name))
    }
}

pub fn feed_animal(animal_name: String) -> Result<String, Box<dyn Error>> {
    let mut reader = reader_animals_data()?;
    if !animal_alive(&animal_name)? {
        Ok(format!("{} is not even alive", animal_name))
    } else if let Some(animal_data) = reader.read_animal_data(animal_name.as_str())? {
        let mut rng = thread_rng();
        let prey = animal_data.preys().choose(&mut rng).unwrap();

        Ok(format!("Mmm... That {} was delicious", prey))
    } else {
        Ok("Animal not found".to_string())
    }
}
pub fn count_animal(animal_name: String) -> Result<String, Box<dyn Error>> {
    let mut reader = reader_animals_alive()?;
    let count = reader.count_animal(animal_name.as_str())?;
    Ok(format!("We have {} {}s!", count, animal_name))
}

pub fn help() -> String {
    "'create animal_name': Create an animal, specifying class, preys and predators
'delete animal_name': Delete an animal data
'read animal_name': Read animal data
'spawn animal_name': Spawn animal of chosen sex
'reproduce animal_name': Reproduce an animal alive
'kill animal_name': Kill an animal alive
'feed animal_name': Feed an animal alive
'count animal_name': Count the number of animals alive
'help': Show this help"
        .to_string()
}
