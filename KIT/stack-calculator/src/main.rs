pub mod calc;

use calc::{Calculator, Command};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut calc = Calculator::new();
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        // '?' passes the error to the main function itself
        // If something is wrong with stdin/stdout then you can crash the program
        io::stdout().flush()?;
        stdin.read_line(&mut input)?;
        if input.trim() == "quit" {
            break;
        }
        calc.execute_command(Command::from_string(&input)?);
        input = String::new();
    }
    Ok(())
}
