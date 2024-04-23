use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
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

pub fn create_writer_append_for_path(path: &str) -> Result<CustomWriter<File>, Box<dyn Error>> {
    let file_write = file_write_append(path)?;
    Ok(CustomWriter::new(csv::Writer::from_writer(file_write)))
}

pub fn create_writer_truncate_for_path(path: &str) -> Result<CustomWriter<File>, Box<dyn Error>> {
    let file_write = file_write_truncate(path)?;
    Ok(CustomWriter::new(csv::Writer::from_writer(file_write)))
}

pub fn create_reader_for_path(path: &str) -> Result<csv::Reader<File>, Box<dyn Error>> {
    let file_read = File::open(path)?;
    Ok(csv::Reader::from_reader(file_read))
}
