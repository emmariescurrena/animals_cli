use csv::Position;
use std::error::Error;
use std::fs::File;
use std::io;

use crate::animal_structs::*;
use crate::common_paths::ANIMALS_DATA_FILE_PATH;
use crate::file_handler::*;

impl AnimalData {
    pub fn to_data_csv(&self) -> [String; 4] {
        [
            self.name(),
            self.class_str(),
            self.preys_str(),
            self.predators_str(),
        ]
    }
}

impl<W: io::Write> CustomWriter<W> {
    pub fn write_animal_data_file(&mut self, animal: AnimalData) -> csv::Result<()> {
        self.inner.write_record(animal.to_data_csv())
    }
}

pub trait AnimalDataReader {
    fn read_animal_data(&mut self, name: &str) -> Result<Option<AnimalData>, Box<dyn Error>>;
    fn animal_data_in_file(&mut self, name: &str) -> Result<bool, Box<dyn Error>>;
}

impl<R: std::io::Read + std::io::Seek> AnimalDataReader for csv::Reader<R> {
    fn read_animal_data(&mut self, name: &str) -> Result<Option<AnimalData>, Box<dyn Error>> {
        for result in self.records() {
            let record = result?;
            if record[0].to_owned() == name {
                let class = match &record[1] {
                    "mammal" => Class::Mammal,
                    "bird" => Class::Bird,
                    "amphibian" => Class::Amphibian,
                    "arthropod" => Class::Arthropod,
                    "fish" => Class::Fish,
                    "reptile" => Class::Reptile,
                    _ => return Err("Invalid class".into()),
                };
                let preys = record[2].split('-').map(String::from).collect::<Vec<_>>();
                let predators = record[3].split('-').map(String::from).collect::<Vec<_>>();
                let animal = AnimalData::new(name, class, predators, preys);
                self.seek(Position::new())?;
                return Ok(Some(animal));
            }
        }
        self.seek(Position::new())?;
        Ok(None)
    }
    fn animal_data_in_file(&mut self, name: &str) -> Result<bool, Box<dyn Error>> {
        if let Some(_animal) = self.read_animal_data(name)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn writer_animals_data() -> Result<CustomWriter<File>, Box<dyn Error>> {
    create_writer_append_for_path(ANIMALS_DATA_FILE_PATH)
}

fn reader_animals_data() -> Result<csv::Reader<File>, Box<dyn Error>> {
    create_reader_for_path(ANIMALS_DATA_FILE_PATH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_handler::*;
    use csv::StringRecord;
    use std::fs::File;

    const TEST_ANIMALS_DATA_FILE_PATH: &str = "test_animals_data.csv";

    fn writer_test_animals_data() -> Result<CustomWriter<File>, Box<dyn Error>> {
        create_writer_append_for_path(TEST_ANIMALS_DATA_FILE_PATH)
    }

    fn reader_test_animals_data() -> Result<csv::Reader<File>, Box<dyn Error>> {
        create_reader_for_path(TEST_ANIMALS_DATA_FILE_PATH)
    }

    fn create_test_animals_csv() -> Result<(), Box<dyn Error>> {
        File::create(TEST_ANIMALS_DATA_FILE_PATH)?;

        let mut writer = create_writer_truncate_for_path(TEST_ANIMALS_DATA_FILE_PATH)?;

        writer
            .inner
            .write_record(&["name", "class", "predators", "preys"])?;
        Ok(())
    }

    #[test]
    fn test_create_csv_for_testing() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;
        let animals_file_read = File::open(TEST_ANIMALS_DATA_FILE_PATH)?;

        let mut reader = csv::Reader::from_reader(animals_file_read);

        for result in reader.records() {
            let record = result?;
            assert_eq!(
                record,
                StringRecord::from(vec!["name", "class", "predators", "preys"])
            );
        }
        Ok(())
    }

    #[test]
    fn test_read_animal_data() -> Result<(), Box<dyn Error>> {
        create_test_animals_csv()?;

        let mut writer = writer_test_animals_data()?;
        writer.write_animal_data_file(AnimalData::new(
            "snake",
            Class::Reptile,
            vec![String::from("eagle"), String::from("mongoose")],
            vec![String::from("rat"), String::from("squirrel")],
        ))?;
        writer.write_animal_data_file(AnimalData::new(
            "chameleon",
            Class::Reptile,
            vec![String::from("bird"), String::from("snake")],
            vec![String::from("mantids"), String::from("crickets")],
        ))?;

        writer.flush()?;

        let animals_file_read = File::open(TEST_ANIMALS_DATA_FILE_PATH)?;

        let mut reader = reader_test_animals_data()?;

        assert!(reader.animal_data_in_file("snake")?);
        assert!(!reader.animal_data_in_file("cow")?);
        assert!(!reader.animal_data_in_file("rabbit")?);
        assert!(reader.animal_data_in_file("chameleon")?);
        Ok(())
    }
}
