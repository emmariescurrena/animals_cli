use std::error::Error;

use crate::custom_writers_and_readers::{create_reader_for_path, create_writer_append_for_path};
use crate::temp_file_handler::*;

fn delete_animal_for_path(
    path: &str,
    animal_name: &str,
    delete_all: bool,
) -> Result<(), Box<dyn Error>> {
    let temp_file = create_temp_file(path)?;
    let temp_file_path = temp_file.path().to_str().unwrap();
    let mut temp_file_writer = create_writer_append_for_path(temp_file_path)?;

    let mut path_reader = create_reader_for_path(path)?;

    temp_file_writer
        .inner
        .write_record(path_reader.inner.headers()?)?;

    let header_name_index = path_reader
        .inner
        .headers()?
        .iter()
        .position(|h| h == "name")
        .unwrap();

    let mut animal_killed = false;
    for result in path_reader.inner.records() {
        let animal_alive = result?;
        if animal_alive.get(header_name_index) != Some(animal_name)
            || (animal_killed && !delete_all)
        {
            temp_file_writer.inner.write_record(&animal_alive)?;
        } else {
            if !animal_killed {
                animal_killed = true;
            }
        }
    }

    temp_file_writer.flush()?;

    std::fs::rename(&temp_file_path, path)?;

    Ok(())
}

pub fn delete_all_animals_for_path(animal_name: &str, path: &str) -> Result<(), Box<dyn Error>> {
    delete_animal_for_path(path, animal_name, true)
}

pub fn delete_one_animal_for_path(animal_name: &str, path: &str) -> Result<(), Box<dyn Error>> {
    delete_animal_for_path(path, animal_name, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animal_structs::animals_alive_models::*;
    use crate::animal_structs::animals_data_models::*;
    use crate::csv_files_creator::{create_test_animals_alive, create_test_animals_data};
    use crate::custom_writers_and_readers::{reader_for_test, writer_for_test};
    use crate::TEST_PATH;

    fn delete_all_animals_for_test(animal_name: &str) -> Result<(), Box<dyn Error>> {
        delete_all_animals_for_path(animal_name, TEST_PATH)?;
        Ok(())
    }

    fn delete_one_animal_for_test(animal_name: &str) -> Result<(), Box<dyn Error>> {
        delete_one_animal_for_path(animal_name, TEST_PATH)?;
        Ok(())
    }

    #[test]
    fn test_delete_all_animals() -> Result<(), Box<dyn Error>> {
        create_test_animals_alive()?;

        let mut writer = writer_for_test()?;

        writer.write_animal_alive(snake_female())?;
        writer.write_animal_alive(chameleon_male())?;
        writer.write_animal_alive(chameleon_male())?;
        writer.write_animal_alive(snake_female())?;

        let mut reader = reader_for_test()?;

        let count_snake = reader.count_animal("snake")?;
        assert_eq!(count_snake, 2);
        let count_chameleon = reader.count_animal("chameleon")?;
        assert_eq!(count_chameleon, 2);

        delete_all_animals_for_test("snake")?;
        reader = reader_for_test()?;

        let count_snake = reader.count_animal("snake")?;
        assert_eq!(count_snake, 0);
        let count_chameleon = reader.count_animal("chameleon")?;
        assert_eq!(count_chameleon, 2);

        Ok(())
    }

    #[test]
    fn test_kill_one_animal_alive() -> Result<(), Box<dyn Error>> {
        create_test_animals_alive()?;

        let mut writer = writer_for_test()?;

        writer.write_animal_alive(snake_female())?;
        writer.write_animal_alive(chameleon_male())?;
        writer.write_animal_alive(chameleon_male())?;
        writer.write_animal_alive(snake_female())?;

        let mut reader = reader_for_test()?;

        let mut count = reader.count_animal("snake")?;
        assert_eq!(count, 2);

        delete_one_animal_for_test("snake")?;

        reader = reader_for_test()?;
        count = reader.count_animal("snake")?;
        assert_eq!(count, 1);

        delete_one_animal_for_test("snake")?;

        reader = reader_for_test()?;
        count = reader.count_animal("snake")?;
        assert_eq!(count, 0);

        Ok(())
    }

    #[test]
    fn test_delete_animal_data() -> Result<(), Box<dyn Error>> {
        create_test_animals_data()?;

        let mut writer = writer_for_test()?;

        writer.write_animal_data(&snake_data())?;
        writer.write_animal_data(&chameleon_data())?;
        writer.write_animal_data(&chameleon_data())?;
        writer.write_animal_data(&snake_data())?;

        let mut reader = reader_for_test()?;

        let mut count = reader.count_animal("snake")?;
        assert_eq!(count, 2);

        delete_one_animal_for_test("snake")?;

        reader = reader_for_test()?;
        count = reader.count_animal("snake")?;
        assert_eq!(count, 1);

        delete_one_animal_for_test("snake")?;

        reader = reader_for_test()?;
        count = reader.count_animal("snake")?;
        assert_eq!(count, 0);

        Ok(())
    }
}
