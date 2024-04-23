extern crate csv;
use csv::Position;
use rand::prelude::{thread_rng, SliceRandom};
use std::error::Error;
use std::fs::File;
use std::io;

use crate::animal_structs::*;
use crate::common_paths::ANIMALS_ALIVE_FILE_PATH;
use crate::file_handler::*;

impl AnimalAlive {
    pub fn to_alive_csv(&self) -> [String; 2] {
        [self.name(), self.sex_str()]
    }
}

impl<W: io::Write> CustomWriter<W> {
    pub fn write_animal_alive(&mut self, animal: AnimalAlive) -> csv::Result<()> {
        self.inner.write_record(animal.to_alive_csv())
    }
    pub fn kill_animal(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let animals_file_read = File::open(ANIMALS_ALIVE_FILE_PATH)?;
        let mut reader = csv::Reader::from_reader(animals_file_read);

        let temp = File::create("temp.csv")?;
        let mut writer = csv::Reader::from_reader(temp);

        Ok(())
    }
}

pub trait AnimalAliveReader {
    fn read_animal_alive(&mut self, name: &str) -> Result<Option<AnimalAlive>, Box<dyn Error>>;
    fn has_both_sexes(&mut self, name: &str) -> Result<bool, Box<dyn Error>>;
    fn count_animal(&mut self, name: &str) -> Result<i32, Box<dyn Error>>;
    fn animal_alive(&mut self, name: &str) -> Result<bool, Box<dyn Error>>;
}

impl<R: std::io::Read + std::io::Seek> AnimalAliveReader for csv::Reader<R> {
    fn read_animal_alive(&mut self, name: &str) -> Result<Option<AnimalAlive>, Box<dyn Error>> {
        for result in self.records() {
            let record = result?;
            if record[0].to_owned() == name {
                let sex = match &record[1] {
                    "male" => Sex::Male,
                    "female" => Sex::Female,
                    _ => return Err("Invalid class".into()),
                };
                let animal = AnimalAlive::born(name, sex);
                self.seek(Position::new())?;
                return Ok(Some(animal));
            }
        }
        self.seek(Position::new())?;
        Ok(None)
    }
    fn has_both_sexes(&mut self, name: &str) -> Result<bool, Box<dyn Error>> {
        let mut male_exists = false;
        let mut female_exists = false;
        for result in self.records() {
            let record = result?;
            if record[0].to_owned() == name {
                match &record[1] {
                    "male" => male_exists = true,
                    "female" => female_exists = true,
                    _ => return Err("Invalid sex".into()),
                }
            }
            if male_exists && female_exists {
                self.seek(Position::new())?;
                return Ok(true);
            }
        }
        self.seek(Position::new())?;
        Ok(false)
    }
    fn count_animal(&mut self, name: &str) -> Result<i32, Box<dyn Error>> {
        let mut count = 0;
        for result in self.records() {
            let record = result?;
            if record[0].to_owned() == name {
                count += 1;
            }
        }
        Ok(count)
    }
    fn animal_alive(&mut self, name: &str) -> Result<bool, Box<dyn Error>> {
        if let Some(_animal) = self.read_animal_alive(name)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn writer_animals_alive() -> Result<CustomWriter<File>, Box<dyn Error>> {
    create_writer_append_for_path(ANIMALS_ALIVE_FILE_PATH)
}

fn reader_animals_alive() -> Result<csv::Reader<File>, Box<dyn Error>> {
    create_reader_for_path(ANIMALS_ALIVE_FILE_PATH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_handler::*;
    use csv::StringRecord;
    use std::fs::File;

    const TEST_ANIMALS_ALIVE_FILE_PATH: &str = "test_animals_alive.csv";

    fn writer_test_animals_alive() -> Result<CustomWriter<File>, Box<dyn Error>> {
        create_writer_append_for_path(TEST_ANIMALS_ALIVE_FILE_PATH)
    }

    fn reader_test_animals_alive() -> Result<csv::Reader<File>, Box<dyn Error>> {
        create_reader_for_path(TEST_ANIMALS_ALIVE_FILE_PATH)
    }

    fn create_test_animals_csv() -> Result<(), Box<dyn Error>> {
        File::create(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let mut writer = create_writer_truncate_for_path(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        writer.inner.write_record(&["name", "sex"])?;
        writer.flush()?;

        Ok(())
    }

    #[test]
    fn test_create_csv_for_testing() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;
        let mut reader = reader_test_animals_alive()?;

        for result in reader.records() {
            let record = result?;
            assert_eq!(record, StringRecord::from(vec!["name", "sex"]));
        }
        Ok(())
    }

    fn snake_female_born() -> AnimalAlive {
        AnimalAlive::born("snake", Sex::Female)
    }
    fn snake_male_born() -> AnimalAlive {
        AnimalAlive::born("snake", Sex::Male)
    }
    fn chameleon_male_born() -> AnimalAlive {
        AnimalAlive::born("chameleon", Sex::Male)
    }

    #[test]
    fn test_has_both_sexes() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;

        let mut writer = writer_test_animals_alive()?;

        writer.write_animal_alive(snake_female_born())?;
        writer.write_animal_alive(snake_male_born())?;
        writer.write_animal_alive(chameleon_male_born())?;
        writer.write_animal_alive(chameleon_male_born())?;

        let mut reader = reader_test_animals_alive()?;

        assert!(reader.has_both_sexes("snake").unwrap());
        assert!(reader.has_both_sexes("chameleon").unwrap());

        Ok(())
    }

    #[test]
    fn test_read_animal() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;

        let mut writer = writer_test_animals_alive()?;

        writer.write_animal_alive(snake_female_born())?;
        writer.write_animal_alive(chameleon_male_born())?;

        let mut reader = reader_test_animals_alive()?;

        assert!(reader.animal_alive("snake")?);
        assert!(!reader.animal_alive("cow")?);
        assert!(!reader.animal_alive("rabbit")?);
        assert!(reader.animal_alive("chameleon")?);
        Ok(())
    }

    #[test]
    fn test_count_animal() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;

        let mut writer = writer_test_animals_alive()?;

        writer.write_animal_alive(snake_female_born())?;
        writer.write_animal_alive(chameleon_male_born())?;
        writer.write_animal_alive(chameleon_male_born())?;
        writer.write_animal_alive(snake_female_born())?;

        let mut reader = reader_test_animals_alive()?;
        let count = reader.count_animal("snake")?;
        assert_eq!(count, 2);
        Ok(())
    }
}
