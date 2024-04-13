#![allow(dead_code)]

use rand::Rng;

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
pub struct Animal {
    name: String,
    class: Class,
    predators: Vec<String>,
    preys: Vec<String>,
    sex: Sex,
}

impl Animal {
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

    pub fn blood_type(&self) -> BloodType {
        match &self.class() {
            Class::Mammal | Class::Bird => BloodType::Warm,
            Class::Amphibian | Class::Arthropod | Class::Fish | Class::Reptile => BloodType::Cold,
        }
    }

    pub fn born(
        name: String,
        class: Class,
        predators: Vec<String>,
        preys: Vec<String>,
        sex: Sex,
    ) -> Self {
        Animal {
            name,
            class,
            predators,
            preys,
            sex,
        }
    }

    pub fn eat(&self, logger: &mut dyn Logger) {
        let mut rng = rand::thread_rng();
        let prey = &self.preys()[rng.gen_range(0..self.preys.len())];
        logger.log(format!("{} found a {} and eated it!", self.name(), prey));
    }

    pub fn reproduce(&self) {
        todo!()
    }

    pub fn die(self) {
        todo!()
    }
}

pub fn cow_model() -> Animal {
    let cow = Animal::born(
        String::from("cow"),
        Class::Mammal,
        vec![String::from("fox"), String::from("human")],
        vec![String::from("grass"), String::from("straw")],
        Sex::Male,
    );
    cow
}

pub fn snake_model() -> Animal {
    let snake = Animal::born(
        String::from("snake"),
        Class::Reptile,
        vec![String::from("eagle"), String::from("mongoose")],
        vec![String::from("rat"), String::from("squirrel")],
        Sex::Female,
    );
    snake
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
        let cow = cow_model();
        let name = cow.name();
        assert_eq!(name, "Cow");
    }

    #[test]
    fn test_class() {
        let cow = cow_model();
        let class = cow.class();
        assert_eq!(class, &Class::Mammal);
    }

    #[test]
    fn test_predators() {
        let cow = cow_model();
        let predators = cow.predators();
        assert_eq!(predators, &vec![String::from("fox"), String::from("human")])
    }

    #[test]
    fn test_predators_str() {
        let cow = cow_model();
        assert_eq!(cow.predators_str(), "fox-human");
    }

    #[test]
    fn test_preys() {
        let cow = cow_model();
        let preys = cow.preys();
        assert_eq!(preys, &vec![String::from("grass"), String::from("straw")])
    }

    #[test]
    fn test_sex() {
        let cow = cow_model();
        let snake = snake_model();
        let cow_sex = cow.sex();
        let snake_sex = snake.sex();
        assert_eq!(cow_sex, &Sex::Male);
        assert_eq!(snake_sex, &Sex::Female);
    }

    #[test]
    fn test_blood_type() {
        let cow = cow_model();
        let snake = snake_model();
        let cow_blood_type = cow.blood_type();
        let snake_blood_type = snake.blood_type();
        assert_eq!(cow_blood_type, BloodType::Warm);
        assert_eq!(snake_blood_type, BloodType::Cold);
    }

    #[test]
    fn test_born() {
        let cow = cow_model();
        assert_eq!(
            cow,
            Animal {
                name: String::from("Cow"),
                class: Class::Mammal,
                predators: vec![String::from("fox"), String::from("human")],
                preys: vec![String::from("grass"), String::from("straw")],
                sex: Sex::Male,
            }
        )
    }

    #[test]
    fn test_eat() {
        let cow = cow_model();
        let mut test_logger = TestLogger::default();
        cow.eat(&mut test_logger);
        let mut equal_words: bool = false;
        for word in ["grass", "straw"] {
            if test_logger.0[0] == format!("Cow found a {} and eated it!", word) {
                equal_words = true;
                break;
            }
        }
        assert!(equal_words);
    }
}
