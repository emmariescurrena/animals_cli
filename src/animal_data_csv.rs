use csv::Position;
use std::error::Error;
use std::fs::File;
use std::io;

use crate::animal_csv_shared_functions::delete_one_animal_for_path;
use crate::animal_structs::*;
use crate::custom_writers_and_readers::*;
use crate::ANIMALS_DATA_FILE_PATH;

impl AnimalData {
    pub fn to_csv(&self) -> [String; 4] {
        [
            self.name(),
            self.class_str(),
            self.predators_str(),
            self.preys_str(),
        ]
    }
}

impl<W: io::Write> CustomWriter<W> {
    pub fn write_animal_data(&mut self, animal: &AnimalData) -> csv::Result<()> {
        self.inner.write_record(animal.to_csv())?;
        self.flush()?;
        Ok(())
    }
}

pub trait AnimalDataReader {
    fn read_animal_data(&mut self, animal_name: &str)
        -> Result<Option<AnimalData>, Box<dyn Error>>;
}

impl<R: std::io::Read + std::io::Seek> AnimalDataReader for CustomReader<R> {
    fn read_animal_data(
        &mut self,
        animal_name: &str,
    ) -> Result<Option<AnimalData>, Box<dyn Error>> {
        for result in self.inner.records() {
            let record = result?;
            if record[0].to_owned() == animal_name {
                let class = match &record[1] {
                    "mammal" => Class::Mammal,
                    "bird" => Class::Bird,
                    "amphibian" => Class::Amphibian,
                    "arthropod" => Class::Arthropod,
                    "fish" => Class::Fish,
                    "reptile" => Class::Reptile,
                    _ => return Err("Invalid class in csv".into()),
                };
                let predators = record[2].split('-').map(String::from).collect::<Vec<_>>();
                let preys = record[3].split('-').map(String::from).collect::<Vec<_>>();
                let animal = AnimalData::new(animal_name.to_string(), class, predators, preys);
                self.inner.seek(Position::new())?;
                return Ok(Some(animal));
            }
        }
        self.seek_to_beginning()?;
        Ok(None)
    }
}

pub fn delete_animal_data(animal_name: &str) -> Result<(), Box<dyn Error>> {
    delete_one_animal_for_path(animal_name, ANIMALS_DATA_FILE_PATH)
}

pub fn writer_animals_data() -> Result<CustomWriter<File>, Box<dyn Error>> {
    create_writer_append_for_path(ANIMALS_DATA_FILE_PATH)
}

pub fn reader_animals_data() -> Result<CustomReader<File>, Box<dyn Error>> {
    create_reader_for_path(ANIMALS_DATA_FILE_PATH)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::animal_structs::animals_data_models::*;
    use crate::csv_files_creator::create_test_animals_data;
    use crate::custom_writers_and_readers::{reader_for_test, writer_for_test};

    #[test]
    fn test_read_animal_data() -> Result<(), Box<dyn Error>> {
        create_test_animals_data()?;

        let mut writer = writer_for_test()?;
        writer.write_animal_data(&snake_data())?;
        writer.write_animal_data(&chameleon_data())?;

        let mut reader = reader_for_test()?;

        assert!(reader.read_animal_data("snake")?.is_some());
        assert!(reader.read_animal_data("cow")?.is_none());
        assert!(reader.read_animal_data("rabbit")?.is_none());
        assert!(reader.read_animal_data("chameleon")?.is_some());
        Ok(())
    }
}
