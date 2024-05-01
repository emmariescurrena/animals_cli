use csv::Position;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;

use crate::TEST_PATH;

fn file_write_append(path: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
}

fn file_write_truncate(path: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

pub struct CustomWriter<W: io::Write> {
    pub inner: csv::Writer<W>,
}

impl<W: io::Write> CustomWriter<W> {
    pub fn new(inner: csv::Writer<W>) -> Self {
        CustomWriter { inner }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub struct CustomReader<R: std::io::Read + std::io::Seek> {
    pub inner: csv::Reader<R>,
}

impl<R: std::io::Read + std::io::Seek> CustomReader<R> {
    pub fn new(inner: csv::Reader<R>) -> Self {
        Self { inner }
    }
    pub fn seek_to_beginning(&mut self) -> Result<(), Box<dyn Error>> {
        self.inner.seek(Position::new())?;
        Ok(())
    }
    pub fn count_animal(&mut self, animal_name: &str) -> Result<i32, Box<dyn Error>> {
        let mut count = 0;
        for result in self.inner.records() {
            let record = result?;
            if record[0].to_owned() == animal_name {
                count += 1;
            }
        }
        self.seek_to_beginning()?;
        Ok(count)
    }
}
pub fn create_reader_for_path(path: &str) -> Result<CustomReader<File>, Box<dyn Error>> {
    let file_read = File::open(path)?;
    Ok(CustomReader::new(csv::Reader::from_reader(file_read)))
}

pub fn create_writer_append_for_path(path: &str) -> Result<CustomWriter<File>, Box<dyn Error>> {
    let file_write = file_write_append(path)?;
    Ok(CustomWriter::new(csv::Writer::from_writer(file_write)))
}

pub fn create_writer_truncate_for_path(path: &str) -> Result<CustomWriter<File>, Box<dyn Error>> {
    let file_write = file_write_truncate(path)?;
    Ok(CustomWriter::new(csv::Writer::from_writer(file_write)))
}

pub fn reader_for_test() -> Result<CustomReader<File>, Box<dyn Error>> {
    create_reader_for_path(TEST_PATH)
}

pub fn writer_for_test() -> Result<CustomWriter<File>, Box<dyn Error>> {
    create_writer_append_for_path(TEST_PATH)
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use super::*;
    use crate::csv_files_creator::create_test_csv;

    #[test]
    fn test_create_reader_for_path() -> Result<(), Box<dyn Error>> {
        create_test_csv(&["header1", "header2"])?;
        let mut _reader = create_reader_for_path(TEST_PATH)?;
        remove_file(TEST_PATH)?;
        Ok(())
    }

    #[test]
    fn test_create_writer_append_for_path() -> Result<(), Box<dyn Error>> {
        create_test_csv(&["header1", "header2"])?;
        let mut _writer = create_writer_append_for_path(TEST_PATH)?;
        remove_file(TEST_PATH)?;
        Ok(())
    }

    #[test]
    fn test_create_writer_truncate_for_path() -> Result<(), Box<dyn Error>> {
        create_test_csv(&["header1", "header2"])?;
        let mut _writer = create_writer_truncate_for_path(TEST_PATH)?;
        remove_file(TEST_PATH)?;
        Ok(())
    }
}
