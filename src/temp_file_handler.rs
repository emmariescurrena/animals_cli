use std::error::Error;
use std::path::Path;
use tempfile::{Builder, NamedTempFile};

fn temp_dir_path_from_path(path: &str) -> Result<&Path, Box<dyn Error>> {
    Path::new(path)
        .parent()
        .ok_or("Failed to get parent directory".into())
}

pub fn create_temp_file(path: &str) -> Result<NamedTempFile, Box<dyn Error>> {
    let temp_dir = temp_dir_path_from_path(path)?;
    Ok(Builder::new().tempfile_in(temp_dir)?)
}
