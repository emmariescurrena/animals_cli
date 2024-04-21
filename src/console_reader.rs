use crate::animal_alive_csv;
use std::error::Error;

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

pub fn create_animal(animal: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
pub fn delete_animal(animal: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
pub fn read_animal(animal: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
pub fn spawn_animal(animal: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
pub fn reproduce_animal(animal: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
pub fn kill_animal(animal: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
pub fn feed_animal(animal: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
pub fn count_animal(animal: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
