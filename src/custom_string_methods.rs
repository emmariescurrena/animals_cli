use std::error::Error;

use crate::animal_structs::Class;

pub trait CustomStringMethods {
    fn to_class(&self) -> Result<Class, Box<dyn Error>>;
}

impl CustomStringMethods for String {
    fn to_class(&self) -> Result<Class, Box<dyn Error>> {
        match self.as_str() {
            "reptile" => Ok(Class::Reptile),
            "mammal" => Ok(Class::Mammal),
            "fish" => Ok(Class::Fish),
            "bird" => Ok(Class::Bird),
            "arthropod" => Ok(Class::Arthropod),
            "amphibian" => Ok(Class::Amphibian),
            _ => return Err("Invalid class conversion".into()),
        }
    }
}
