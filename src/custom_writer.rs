use std::io::{Result, Write};
pub struct CustomWriter<W: Write> {
    pub inner: csv::Writer<W>,
}

impl<W: Write> CustomWriter<W> {
    pub fn new(inner: csv::Writer<W>) -> Self {
        CustomWriter { inner }
    }

    pub fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
