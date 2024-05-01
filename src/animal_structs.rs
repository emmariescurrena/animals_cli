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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Sex {
    Male,
    Female,
}

#[derive(PartialEq, Debug)]
pub struct AnimalData {
    pub name: String,
    pub class: Class,
    pub predators: Vec<String>,
    pub preys: Vec<String>,
}

impl AnimalData {
    pub fn new(name: String, class: Class, predators: Vec<String>, preys: Vec<String>) -> Self {
        AnimalData {
            name,
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

pub mod animals_data_models {
    use super::*;
    pub fn snake_data() -> AnimalData {
        AnimalData::new(
            "snake".to_string(),
            Class::Reptile,
            vec![String::from("eagle"), String::from("mongoose")],
            vec![String::from("rat"), String::from("squirrel")],
        )
    }
    pub fn chameleon_data() -> AnimalData {
        AnimalData::new(
            "chameleon".to_string(),
            Class::Reptile,
            vec![String::from("bird"), String::from("snake")],
            vec![String::from("mantids"), String::from("crickets")],
        )
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
}

pub mod animals_alive_models {
    use super::*;
    pub fn snake_female() -> AnimalAlive {
        AnimalAlive::born("snake", Sex::Female)
    }
    pub fn snake_male() -> AnimalAlive {
        AnimalAlive::born("snake", Sex::Male)
    }
    pub fn chameleon_male() -> AnimalAlive {
        AnimalAlive::born("chameleon", Sex::Male)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use tests::animals_alive_models::snake_female;
    use tests::animals_data_models::snake_data;

    #[test]
    fn test_class() {
        let snake = snake_data();
        let class = snake.class();
        assert_eq!(class, &Class::Reptile);
    }

    #[test]
    fn test_predators() {
        let snake = snake_data();
        let predators = snake.predators();
        assert_eq!(
            predators,
            &vec![String::from("eagle"), String::from("mongoose")]
        )
    }

    #[test]
    fn test_predators_str() {
        let snake = snake_data();
        assert_eq!(snake.predators_str(), "eagle-mongoose");
    }

    #[test]
    fn test_preys() {
        let snake = snake_data();
        let preys = snake.preys();
        assert_eq!(preys, &vec![String::from("rat"), String::from("squirrel")])
    }

    #[test]
    fn test_blood_type() {
        let snake = snake_data();
        let snake_blood_type = snake.blood_type();
        assert_eq!(snake_blood_type, BloodType::Cold);
    }

    #[test]
    fn test_name() {
        let snake = snake_female();
        let name = snake.name();
        assert_eq!(name, "snake");
    }

    #[test]
    fn test_sex() {
        let snake = snake_female();
        let snake_sex = snake.sex();
        assert_eq!(snake_sex, &Sex::Female);
    }
}
