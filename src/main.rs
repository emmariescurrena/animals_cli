use animals_cli::clear_console::clear_terminal_screen;
use animals_cli::csv_files_creator::*;
use animals_cli::input_handler::handle_inputs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    create_animals_data_if_not_exists()?;
    create_animals_alive_if_not_exists()?;
    clear_terminal_screen();
    handle_inputs()?;

    Ok(())
}
