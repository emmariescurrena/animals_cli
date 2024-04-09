use rand::Rng;

trait Logger {
    fn log(&mut self, value: String);
}
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, value: String) {
        println!("{}", value);
    }
}

#[derive(PartialEq, Debug)]
enum BloodType {
    Warm,
    Cold,
}

#[derive(PartialEq, Debug)]
enum Class {
    Reptile,
    Mammal,
    Fish,
    Bird,
    Arthropod,
    Amphibian,
}

#[derive(PartialEq, Debug)]
enum Sex {
    Male,
    Female,
}

#[derive(PartialEq, Debug)]
struct Animal {
    name: String,
    class: Class,
    predators: Vec<String>,
    preys: Vec<String>,
    sex: Sex,
}

impl Animal {
    fn name(&self) -> &str {
        &self.name
    }
    fn class(&self) -> &Class {
        &self.class
    }
    fn predators(&self) -> &Vec<String> {
        &self.predators
    }
    fn preys(&self) -> &Vec<String> {
        &self.preys
    }
    fn sex(&self) -> &Sex {
        &self.sex
    }
    fn blood_type(&self) -> BloodType {
        match &self.class() {
            Class::Mammal | Class::Bird => BloodType::Warm,
            Class::Amphibian | Class::Arthropod | Class::Fish | Class::Reptile => BloodType::Cold,
        }
    }
    fn born(
        name: String,
        class: Class,
        predators: Vec<String>,
        preys: Vec<String>,
        sex: Sex,
    ) -> Self {
        Animal {
            name: name,
            class: class,
            predators: predators,
            preys: preys,
            sex: sex,
        }
    }
    fn eat(&self, logger: &mut dyn Logger) {
        let mut rng = rand::thread_rng();
        let prey = &self.preys()[rng.gen_range(0..self.preys.len())];
        logger.log(format!("{} found a {} and eated it!", self.name(), prey));
    }
    fn reproduce(&self) {
        todo!()
    }
    fn die(self) {
        todo!()
    }
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

    fn cow_for_testing() -> Animal {
        let cow = Animal::born(
            String::from("Cow"),
            Class::Mammal,
            vec![String::from("fox"), String::from("human")],
            vec![String::from("grass"), String::from("straw")],
            Sex::Male,
        );
        cow
    }

    fn snake_for_testing() -> Animal {
        let snake = Animal::born(
            String::from("Snake"),
            Class::Reptile,
            vec![String::from("eagle"), String::from("mongoose")],
            vec![String::from("rat"), String::from("squirrel")],
            Sex::Female,
        );
        snake
    }

    #[test]
    fn test_animal_name() {
        let cow = cow_for_testing();
        let name = cow.name();
        assert_eq!(name, "Cow");
    }
    #[test]
    fn test_animal_class() {
        let cow = cow_for_testing();
        let class = cow.class();
        assert_eq!(class, &Class::Mammal);
    }

    #[test]
    fn test_animal_predators() {
        let cow = cow_for_testing();
        let predators = cow.predators();
        assert_eq!(predators, &vec![String::from("fox"), String::from("human")])
    }

    #[test]
    fn test_animal_preys() {
        let cow = cow_for_testing();
        let preys = cow.preys();
        assert_eq!(preys, &vec![String::from("grass"), String::from("straw")])
    }

    #[test]
    fn test_animal_sex() {
        let cow = cow_for_testing();
        let snake = snake_for_testing();
        let cow_sex = cow.sex();
        let snake_sex = snake.sex();
        assert_eq!(cow_sex, &Sex::Male);
        assert_eq!(snake_sex, &Sex::Female);
    }

    #[test]
    fn test_animal_blood_type() {
        let cow = cow_for_testing();
        let snake = snake_for_testing();
        let cow_blood_type = cow.blood_type();
        let snake_blood_type = snake.blood_type();
        assert_eq!(cow_blood_type, BloodType::Warm);
        assert_eq!(snake_blood_type, BloodType::Cold);
    }

    #[test]
    fn test_animal_born() {
        let cow = cow_for_testing();
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
    fn test_animal_eat() {
        let cow = cow_for_testing();
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
