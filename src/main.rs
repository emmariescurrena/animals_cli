mod animals;
mod animals_csv;

use animals::*;
use animals_csv::*;
use std::error::Error;
use std::fs::{File, OpenOptions};

fn main() -> Result<(), Box<dyn Error>> {
    // Open the file with read and write permissions
    let animals_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("animals.csv")?;

    // Create the writer
    let mut writer = CustomWriter::new(csv::Writer::from_writer(animals_file.try_clone()?));

    // Write the snake record
    let snake = snake_model();
    writer.write_animal_csv(snake)?;

    // Reopen the file for reading
    let animals_file = File::open("animals.csv")?;
    let mut reader = csv::Reader::from_reader(animals_file);

    // Search for the snake record
    if let Some(snake) = reader.read_animal("snake")? {
        println!("{:?}", snake);
    } else {
        println!("Snake not found in the CSV file.");
    }

    // Search for the cow record
    if let Some(cow) = reader.read_animal("cow")? {
        println!("{:?}", cow);
    } else {
        println!("Cow not found in the CSV file.");
    }

    Ok(())
}
