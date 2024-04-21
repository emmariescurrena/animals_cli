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
            Class::Amphibian | Class::Arthropod | Class::Fish | Class::Reptile => BloodType::Cold,
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
