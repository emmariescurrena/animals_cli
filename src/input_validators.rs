use regex::Regex;

fn valid_string_lowercased_with_hyphens(input: &String) -> bool {
    let pattern = Regex::new(r"^[a-z]+(?:-[a-z]+)*$").unwrap();
    pattern.is_match(input)
}

pub fn valid_class(input: &String) -> bool {
    [
        "reptile",
        "mammal",
        "fish",
        "bird",
        "arthropod",
        "amphibian",
    ]
    .contains(&input.as_str())
}

pub fn valid_animal_name(input: &String) -> bool {
    valid_string_lowercased_with_hyphens(input)
}

pub fn valid_predators(input: &String) -> bool {
    valid_string_lowercased_with_hyphens(input)
}
pub fn valid_preys(input: &String) -> bool {
    valid_string_lowercased_with_hyphens(input)
}

pub fn any_input_is_valid(_input: &String) -> bool {
    true
}

pub fn valid_sex(input: &String) -> bool {
    ["m", "f"].contains(&input.as_str())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::input_validators::valid_sex;

    use super::*;

    #[test]
    fn test_string_lowercased_with_hyphens() -> Result<(), Box<dyn Error>> {
        assert!(valid_predators(&"shark-jelly".to_string()));
        assert!(valid_predators(&"rat".to_string()));
        assert!(valid_predators(&"elephant-lion-zebra".to_string()));

        assert!(!valid_predators(&"-shark".to_string()));
        assert!(!valid_predators(&"shark-".to_string()));
        assert!(!valid_predators(&"shark-lion-".to_string()));
        assert!(!valid_predators(&"-lion-shark".to_string()));
        assert!(!valid_predators(&"lion8-shark".to_string()));
        assert!(!valid_predators(&"".to_string()));
        assert!(!valid_predators(&"-".to_string()));
        Ok(())
    }

    #[test]
    fn test_valid_class() -> Result<(), Box<dyn Error>> {
        assert!(valid_class(&"reptile".to_string()));
        assert!(valid_class(&"mammal".to_string()));
        assert!(valid_class(&"fish".to_string()));
        assert!(valid_class(&"bird".to_string()));
        assert!(valid_class(&"arthropod".to_string()));
        assert!(valid_class(&"amphibian".to_string()));

        assert!(!valid_class(&"reptil".to_string()));
        assert!(!valid_class(&"mammals".to_string()));
        assert!(!valid_class(&"fishes".to_string()));
        assert!(!valid_class(&"birds".to_string()));
        assert!(!valid_class(&"arthropods".to_string()));
        assert!(!valid_class(&"amphibians".to_string()));
        Ok(())
    }

    #[test]
    fn test_valid_sex() -> Result<(), Box<dyn Error>> {
        assert!(valid_sex(&"m".to_string()));
        assert!(valid_sex(&"f".to_string()));

        assert!(!valid_sex(&"female".to_string()));
        assert!(!valid_sex(&"male".to_string()));
        Ok(())
    }
}
