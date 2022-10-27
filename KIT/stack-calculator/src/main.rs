pub mod calc;

use calc::{Calculator, Command};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut calc = Calculator::new();
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
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
