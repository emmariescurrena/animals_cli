use csv::Position;
use std::error::Error;
use std::fs::File;
use std::io;

use crate::animal_csv_shared_functions::{delete_all_animals_for_path, delete_one_animal_for_path};
use crate::animal_structs::*;
use crate::custom_writers_and_readers::*;
use crate::ANIMALS_ALIVE_FILE_PATH;

impl AnimalAlive {
    pub fn to_csv(&self) -> [String; 2] {
        [self.name(), self.sex_str()]
    }
}

impl<W: io::Write> CustomWriter<W> {
    pub fn write_animal_alive(&mut self, animal: AnimalAlive) -> csv::Result<()> {
        self.inner.write_record(animal.to_csv())?;
        self.flush()?;
        Ok(())
    }
}

pub trait AnimalAliveReader {
    fn read_animal_alive(
        &mut self,
        animal_name: &str,
    ) -> Result<Option<AnimalAlive>, Box<dyn Error>>;
    fn has_both_sexes(&mut self, animal_name: &str) -> Result<bool, Box<dyn Error>>;
    fn count_animal(&mut self, animal_name: &str) -> Result<i32, Box<dyn Error>>;
}

impl<R: std::io::Read + std::io::Seek> AnimalAliveReader for CustomReader<R> {
    fn read_animal_alive(
        &mut self,
        animal_name: &str,
    ) -> Result<Option<AnimalAlive>, Box<dyn Error>> {
        for result in self.inner.records() {
            let record = result?;
            if record[0].to_owned() == animal_name {
                let sex = match &record[1] {
                    "male" => Sex::Male,
                    "female" => Sex::Female,
                    _ => return Err("Invalid sex".into()),
                };
                let animal = AnimalAlive::born(animal_name, sex);
                self.seek_to_beginning()?;
                return Ok(Some(animal));
            }
        }
        self.seek_to_beginning()?;
        Ok(None)
    }
    fn has_both_sexes(&mut self, animal_name: &str) -> Result<bool, Box<dyn Error>> {
        let mut male_exists = false;
        let mut female_exists = false;
        for result in self.inner.records() {
            let record = result?;
            if record[0].to_owned() == animal_name {
                match &record[1] {
                    "male" => male_exists = true,
                    "female" => female_exists = true,
                    _ => return Err("Invalid sex".into()),
                }
            }
            if male_exists && female_exists {
                self.inner.seek(Position::new())?;
                return Ok(true);
            }
        }
        self.seek_to_beginning()?;
        Ok(false)
    }
    fn count_animal(&mut self, animal_name: &str) -> Result<i32, Box<dyn Error>> {
        self.count_animal(animal_name)
    }
}

pub fn kill_all_animals_alive(animal_name: &str) -> Result<(), Box<dyn Error>> {
    delete_all_animals_for_path(animal_name, ANIMALS_ALIVE_FILE_PATH)
}

pub fn kill_one_animal_alive(animal_name: &str) -> Result<(), Box<dyn Error>> {
    delete_one_animal_for_path(animal_name, ANIMALS_ALIVE_FILE_PATH)
}

pub fn writer_animals_alive() -> Result<CustomWriter<File>, Box<dyn Error>> {
    create_writer_append_for_path(ANIMALS_ALIVE_FILE_PATH)
}

pub fn reader_animals_alive() -> Result<CustomReader<File>, Box<dyn Error>> {
    create_reader_for_path(ANIMALS_ALIVE_FILE_PATH)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::animal_structs::animals_alive_models::*;
    use crate::csv_files_creator::create_test_animals_alive;
    use crate::custom_writers_and_readers::{reader_for_test, writer_for_test};

    #[test]
    fn test_read_animal_alive() -> Result<(), Box<dyn Error>> {
        create_test_animals_alive()?;

        let mut writer = writer_for_test()?;

        writer.write_animal_alive(snake_female())?;
        writer.write_animal_alive(chameleon_male())?;

        let mut reader = reader_for_test()?;

        assert!(reader.read_animal_alive("snake")?.is_some());
        assert!(reader.read_animal_alive("rabbit")?.is_none());
        assert!(reader.read_animal_alive("cow")?.is_none());
        assert!(reader.read_animal_alive("chameleon")?.is_some());
        Ok(())
    }

    #[test]
    fn test_has_both_sexes() -> Result<(), Box<dyn Error>> {
        create_test_animals_alive()?;

        let mut writer = writer_for_test()?;

        writer.write_animal_alive(snake_female())?;
        writer.write_animal_alive(snake_male())?;
        writer.write_animal_alive(chameleon_male())?;
        writer.write_animal_alive(chameleon_male())?;

        let mut reader = reader_for_test()?;

        assert!(reader.has_both_sexes("snake")?);
        assert!(!reader.has_both_sexes("chameleon")?);

        Ok(())
    }
}
