use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::custom_writers_and_readers::create_writer_truncate_for_path;
use crate::{ANIMALS_ALIVE_FILE_PATH, ANIMALS_DATA_FILE_PATH, TEST_PATH};

pub fn create_animals_data_if_not_exists() -> Result<(), Box<dyn Error>> {
    create_csv_if_not_exists(
        ANIMALS_DATA_FILE_PATH,
        &["name", "class", "predators", "preys"],
    )?;
    Ok(())
}

pub fn create_animals_alive_if_not_exists() -> Result<(), Box<dyn Error>> {
    create_csv_if_not_exists(ANIMALS_ALIVE_FILE_PATH, &["name", "sex"])?;
    Ok(())
}

fn create_csv_if_not_exists(filename: &str, headers: &[&str]) -> Result<(), Box<dyn Error>> {
    let path = Path::new(filename);

    if !path.exists() {
        let mut writer = create_writer_truncate_for_path(filename)?;

        writer.inner.write_record(headers)?;
    }
    Ok(())
}

pub fn create_test_csv(headers: &[&str]) -> Result<(), Box<dyn Error>> {
    File::create(TEST_PATH)?;

    let mut writer = create_writer_truncate_for_path(TEST_PATH)?;

    writer.inner.write_record(headers)?;
    Ok(())
}

pub fn create_test_animals_alive() -> Result<(), Box<dyn Error>> {
    create_test_csv(&["name", "sex"])
}

pub fn create_test_animals_data() -> Result<(), Box<dyn Error>> {
    create_test_csv(&["name", "class", "predators", "preys"])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_writers_and_readers::create_reader_for_path;
    use csv::StringRecord;
    use std::fs::remove_file;

    #[test]
    fn test_create_csv_if_not_exists() -> Result<(), Box<dyn Error>> {
        create_csv_if_not_exists(TEST_PATH, &["header1", "header2"])?;
        let mut reader = create_reader_for_path(TEST_PATH)?;

        for result in reader.inner.records() {
            let record = result?;
            assert_eq!(record, StringRecord::from(vec!["header1", "header2"]));
        }
        remove_file(TEST_PATH)?;
        Ok(())
    }

    #[test]
    fn test_create_csv_for_testing() -> Result<(), Box<dyn Error>> {
        create_test_csv(&["header1", "header2"])?;
        let mut reader = create_reader_for_path(TEST_PATH)?;

        for result in reader.inner.records() {
            let record = result?;
            assert_eq!(record, StringRecord::from(vec!["header1", "header2"]));
        }
        remove_file(TEST_PATH)?;
        Ok(())
    }
}
