use animals_cli::clear_console::clear_terminal_screen;
use animals_cli::commands_handler::handle_input;
use animals_cli::csv_verifiers::*;
use std::error::Error;
use text_io::scan;

fn main() -> Result<(), Box<dyn Error>> {
    create_animals_data_if_not_exists()?;
    create_animals_alive_if_not_exists()?;
    clear_terminal_screen();
    loop {
        let command_name: String;
        let argument: String;
        println!("Insert a command or type help to see commands available");
        scan!("{} {}", command_name, argument);
        clear_terminal_screen();
        handle_input(command_name.as_str(), argument.as_str())?;
    }
    Ok(())
}
