use std::fs::{File, OpenOptions};
use std::io::Error;

pub fn animals_file_write_append(path: &str) -> Result<File, Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
}

pub fn animals_file_write_truncate(path: &str) -> Result<File, Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}
