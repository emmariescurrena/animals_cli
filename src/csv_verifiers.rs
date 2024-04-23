use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::common_paths::*;
use crate::custom_writer::CustomWriter;
use crate::file_handler::animals_file_write_truncate;

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

fn create_csv_if_not_exists(filename: &str, headers: &[&str]) -> std::io::Result<()> {
    let path = Path::new(filename);

    if !path.exists() {
        let animals_file_write = animals_file_write_truncate(filename)?;

        let mut writer = CustomWriter::new(csv::Writer::from_writer(animals_file_write));

        writer.inner.write_record(headers)?;
    }
    Ok(())
}
