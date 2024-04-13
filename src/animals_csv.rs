extern crate csv;

use crate::animals::*;
use std::error::Error;
use std::io;

impl Animal {
    pub fn to_csv(&self) -> [String; 5] {
        [
            self.name(),
            self.class_str(),
            self.preys_str(),
            self.predators_str(),
            self.sex_str(),
        ]
    }
}

pub struct CustomWriter<W: io::Write> {
    inner: csv::Writer<W>,
}

impl<W: io::Write> CustomWriter<W> {
    pub fn new(inner: csv::Writer<W>) -> Self {
        CustomWriter { inner }
    }

    pub fn write_animal_csv(&mut self, animal: Animal) -> csv::Result<()> {
        self.inner.write_record(animal.to_csv())
    }
}

pub trait AnimalReader {
    fn read_animal(&mut self, name: &str) -> Result<Option<Animal>, Box<dyn Error>>;
}

impl<R: std::io::Read> AnimalReader for csv::Reader<R> {
    fn read_animal(&mut self, name: &str) -> Result<Option<Animal>, Box<dyn Error>> {
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
                    _ => panic!("Invalid class"),
                };
                let preys = record[2].split('-').map(String::from).collect::<Vec<_>>();
                let predators = record[3].split('-').map(String::from).collect::<Vec<_>>();
                let sex = match &record[4] {
                    "male" => Sex::Male,
                    "female" => Sex::Female,
                    _ => panic!("Invalid sex"),
                };
                let animal = Animal::born(name.to_string(), class, predators, preys, sex);
                return Ok(Some(animal));
            }
        }
        Ok(None)
    }
}
