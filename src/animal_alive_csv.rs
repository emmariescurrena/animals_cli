extern crate csv;
use crate::animal_structs::*;
use crate::custom_writer::CustomWriter;
use csv::Position;
use std::error::Error;
use std::io;

impl AnimalAlive {
    pub fn to_alive_csv(&self) -> [String; 2] {
        [self.name(), self.sex_str()]
    }
}

impl<W: io::Write> CustomWriter<W> {
    pub fn write_animal_alive_file(&mut self, animal: AnimalAlive) -> csv::Result<()> {
        self.inner.write_record(animal.to_alive_csv())
    }
}

pub trait AnimalReader {
    fn read_animal_alive(&mut self, name: &str) -> Result<Option<AnimalAlive>, Box<dyn Error>>;
    fn has_both_sexes(&mut self, name: &str) -> Result<bool, Box<dyn Error>>;
    fn count_animal(&mut self, name: &str) -> Result<i32, Box<dyn Error>>;
    fn animal_alive_in_file(&mut self, name: &str) -> Result<bool, Box<dyn Error>>;
}

impl<R: std::io::Read + std::io::Seek> AnimalReader for csv::Reader<R> {
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
    fn animal_alive_in_file(&mut self, name: &str) -> Result<bool, Box<dyn Error>> {
        if let Some(_animal) = self.read_animal_alive(name)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_handler::*;
    use csv::StringRecord;
    use std::fs::File;

    const TEST_ANIMALS_ALIVE_FILE_PATH: &str = "test_animals_alive.csv";

    fn create_test_animals_csv() -> Result<(), Box<dyn Error>> {
        File::create(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let animals_file_write = animals_file_write_truncate(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let mut writer = CustomWriter::new(csv::Writer::from_writer(animals_file_write));

        writer.inner.write_record(&["name", "sex"])?;
        writer.flush()?;

        Ok(())
    }

    #[test]
    fn test_create_csv_for_testing() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;
        let animals_file_read = File::open(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let mut reader = csv::Reader::from_reader(animals_file_read);

        for result in reader.records() {
            let record = result?;
            assert_eq!(record, StringRecord::from(vec!["name", "sex"]));
        }
        Ok(())
    }

    #[test]
    fn test_has_both_sexes() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;
        let animals_file_write = animals_file_write_append(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let mut writer = CustomWriter::new(csv::Writer::from_writer(animals_file_write));

        writer.write_animal_alive_file(AnimalAlive::born("snake", Sex::Female))?;
        writer.write_animal_alive_file(AnimalAlive::born("snake", Sex::Male))?;
        writer.write_animal_alive_file(AnimalAlive::born("chameleon", Sex::Male))?;
        writer.write_animal_alive_file(AnimalAlive::born("chameleon", Sex::Male))?;

        writer.flush()?;

        let animals_file_read = File::open(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let mut reader = csv::Reader::from_reader(animals_file_read);

        assert!(reader.has_both_sexes("snake").unwrap());
        assert!(!reader.has_both_sexes("chameleon").unwrap());

        Ok(())
    }

    #[test]
    fn test_read_animal() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;

        let animals_file_write = animals_file_write_append(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let mut writer = CustomWriter::new(csv::Writer::from_writer(animals_file_write));
        writer.write_animal_alive_file(AnimalAlive::born("snake", Sex::Female))?;
        writer.write_animal_alive_file(AnimalAlive::born("chameleon", Sex::Male))?;

        writer.flush()?;

        let animals_file_read = File::open(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let mut reader = csv::Reader::from_reader(animals_file_read);

        assert!(reader.animal_alive_in_file("snake").unwrap());
        assert!(!reader.animal_alive_in_file("cow").unwrap());
        assert!(!reader.animal_alive_in_file("rabbit").unwrap());
        assert!(reader.animal_alive_in_file("chameleon").unwrap());
        Ok(())
    }

    #[test]
    fn test_count_animal() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;

        let animals_file_write = animals_file_write_append(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let mut writer = CustomWriter::new(csv::Writer::from_writer(animals_file_write));

        fn snake_born() -> AnimalAlive {
            AnimalAlive::born("snake", Sex::Female)
        }
        fn chameleon_born() -> AnimalAlive {
            AnimalAlive::born("chameleon", Sex::Female)
        }

        writer.write_animal_alive_file(snake_born())?;
        writer.write_animal_alive_file(chameleon_born())?;
        writer.write_animal_alive_file(chameleon_born())?;
        writer.write_animal_alive_file(snake_born())?;

        writer.flush()?;

        let animals_file_read = File::open(TEST_ANIMALS_ALIVE_FILE_PATH)?;

        let mut reader = csv::Reader::from_reader(animals_file_read);

        let count = reader.count_animal("snake");
        assert_eq!(count.unwrap(), 2);
        Ok(())
    }
}
