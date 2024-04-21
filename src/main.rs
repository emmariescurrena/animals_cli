use animals_cli::console_reader::read_command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    read_command()?;
    Ok(())
}
