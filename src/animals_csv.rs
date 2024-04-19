pub mod file_handler {
    use std::fs::{File, OpenOptions};
    use std::io::Error;

    pub fn animals_file_write_append(path: &str) -> Result<File, Error> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)
    }

    pub fn animals_file_write_truncate(path: &str) -> Result<File, Error> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
    }
}

pub mod animal_structs {
    // use rand::Rng;

    pub trait Logger {
        fn log(&mut self, value: String);
    }

    struct ConsoleLogger;

    impl Logger for ConsoleLogger {
        fn log(&mut self, value: String) {
            println!("{}", value);
        }
    }

    #[derive(PartialEq, Debug)]
    pub enum BloodType {
        Warm,
        Cold,
    }

    #[derive(PartialEq, Debug)]
    pub enum Class {
        Reptile,
        Mammal,
        Fish,
        Bird,
        Arthropod,
        Amphibian,
    }

    #[derive(PartialEq, Debug)]
    pub enum Sex {
        Male,
        Female,
    }

    #[derive(PartialEq, Debug)]
    pub struct AnimalData {
        name: String,
        class: Class,
        predators: Vec<String>,
        preys: Vec<String>,
    }

    impl AnimalData {
        pub fn new(name: &str, class: Class, predators: Vec<String>, preys: Vec<String>) -> Self {
            AnimalData {
                name: name.to_string(),
                class,
                predators,
                preys,
            }
        }
        pub fn name(&self) -> String {
            self.name.clone()
        }

        pub fn class(&self) -> &Class {
            &self.class
        }

        pub fn class_str(&self) -> String {
            let class = match &self.class() {
                Class::Mammal => "mammal",
                Class::Bird => "bird",
                Class::Amphibian => "amphibian",
                Class::Arthropod => "arthropod",
                Class::Fish => "fish",
                Class::Reptile => "reptile",
            };
            class.to_string()
        }

        pub fn predators(&self) -> &Vec<String> {
            &self.predators
        }

        pub fn predators_str(&self) -> String {
            self.predators.join("-")
        }

        pub fn preys(&self) -> &Vec<String> {
            &self.preys
        }

        pub fn preys_str(&self) -> String {
            self.preys.join("-")
        }

        pub fn blood_type(&self) -> BloodType {
            match &self.class() {
                Class::Mammal | Class::Bird => BloodType::Warm,
                Class::Amphibian | Class::Arthropod | Class::Fish | Class::Reptile => {
                    BloodType::Cold
                }
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct AnimalAlive {
        name: String,
        sex: Sex,
    }

    impl AnimalAlive {
        pub fn name(&self) -> String {
            self.name.clone()
        }

        pub fn sex(&self) -> &Sex {
            &self.sex
        }

        pub fn sex_str(&self) -> String {
            let sex = match &self.sex() {
                Sex::Male => "male",
                Sex::Female => "female",
            };
            sex.to_string()
        }

        pub fn born(name: &str, sex: Sex) -> AnimalAlive {
            AnimalAlive {
                name: name.to_string(),
                sex,
            }
        }

        /* pub fn eat(&self, logger: &mut dyn Logger) {
            let mut rng = rand::thread_rng();
            let prey = &self.preys()[rng.gen_range(0..self.preys.len())];
            logger.log(format!("{} found a {} and eated it!", self.name(), prey));
        }
        */
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[derive(Default)]
        struct TestLogger(Vec<String>);
        impl Logger for TestLogger {
            fn log(&mut self, value: String) {
                self.0.push(value);
            }
        }

        #[test]
        fn test_name() {
            let snake = AnimalAlive::born("snake", Sex::Female);
            let name = snake.name();
            assert_eq!(name, "snake");
        }

        #[test]
        fn test_class() {
            let snake = AnimalData::new(
                "snake",
                Class::Reptile,
                vec![String::from("eagle"), String::from("mongoose")],
                vec![String::from("rat"), String::from("squirrel")],
            );
            let class = snake.class();
            assert_eq!(class, &Class::Reptile);
        }

        #[test]
        fn test_predators() {
            let snake = AnimalData::new(
                "snake",
                Class::Reptile,
                vec![String::from("eagle"), String::from("mongoose")],
                vec![String::from("rat"), String::from("squirrel")],
            );
            let predators = snake.predators();
            assert_eq!(
                predators,
                &vec![String::from("eagle"), String::from("mongoose")]
            )
        }

        #[test]
        fn test_predators_str() {
            let snake = AnimalData::new(
                "snake",
                Class::Reptile,
                vec![String::from("eagle"), String::from("mongoose")],
                vec![String::from("rat"), String::from("squirrel")],
            );
            assert_eq!(snake.predators_str(), "eagle-mongoose");
        }

        #[test]
        fn test_preys() {
            let snake = AnimalData::new(
                "snake",
                Class::Reptile,
                vec![String::from("eagle"), String::from("mongoose")],
                vec![String::from("rat"), String::from("squirrel")],
            );
            let preys = snake.preys();
            assert_eq!(preys, &vec![String::from("rat"), String::from("squirrel")])
        }

        #[test]
        fn test_sex() {
            let snake = AnimalAlive::born("snake", Sex::Female);
            let snake_sex = snake.sex();
            assert_eq!(snake_sex, &Sex::Female);
        }

        #[test]
        fn test_blood_type() {
            let snake = AnimalData::new(
                "snake",
                Class::Reptile,
                vec![String::from("eagle"), String::from("mongoose")],
                vec![String::from("rat"), String::from("squirrel")],
            );
            let snake_blood_type = snake.blood_type();
            assert_eq!(snake_blood_type, BloodType::Cold);
        }

        #[test]
        fn test_born() {
            let snake = AnimalAlive::born("snake", Sex::Female);
            assert_eq!(
                snake,
                AnimalAlive {
                    name: String::from("snake"),
                    sex: Sex::Female,
                }
            )
        }
    }
}

pub mod custom_writer {
    use std::io::{Result, Write};
    pub struct CustomWriter<W: Write> {
        pub inner: csv::Writer<W>,
    }

    impl<W: Write> CustomWriter<W> {
        pub fn new(inner: csv::Writer<W>) -> Self {
            CustomWriter { inner }
        }

        pub fn flush(&mut self) -> Result<()> {
            self.inner.flush()
        }
    }
}

pub mod animal_data_csv {

    use crate::animal_structs::*;
    use crate::custom_writer::CustomWriter;

    use csv::Position;
    use std::error::Error;
    use std::io;

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

    pub trait AnimalReader {
        fn read_animal_data(&mut self, name: &str) -> Result<Option<AnimalData>, Box<dyn Error>>;
        fn animal_data_in_file(&mut self, name: &str) -> Result<bool, Box<dyn Error>>;
    }

    impl<R: std::io::Read + std::io::Seek> AnimalReader for csv::Reader<R> {
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::file_handler::*;
        use csv::StringRecord;
        use std::fs::File;

        const TEST_ANIMALS_DATA_FILE_PATH: &str = "test_animals_data.csv";

        fn create_test_animals_csv() -> Result<(), Box<dyn Error>> {
            File::create(TEST_ANIMALS_DATA_FILE_PATH)?;

            let animals_file_write = animals_file_write_truncate(TEST_ANIMALS_DATA_FILE_PATH)?;

            let mut writer = CustomWriter::new(csv::Writer::from_writer(animals_file_write));

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

            let animals_file_write = animals_file_write_append(TEST_ANIMALS_DATA_FILE_PATH)?;

            let mut writer = CustomWriter::new(csv::Writer::from_writer(animals_file_write));
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

            let mut reader = csv::Reader::from_reader(animals_file_read);

            assert!(reader.animal_data_in_file("snake").unwrap());
            assert!(!reader.animal_data_in_file("cow").unwrap());
            assert!(!reader.animal_data_in_file("rabbit").unwrap());
            assert!(reader.animal_data_in_file("chameleon").unwrap());
            Ok(())
        }
    }
}

pub mod animal_alive_csv {

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
}
