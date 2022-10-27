pub mod calc;

use calc::{Calculator, Command};
use std::io::{self, Write};

fn main() {
    let mut calc = Calculator::new();
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        _ = io::stdout().flush();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                if input.trim().eq("quit") {
                    break;
                }
                calc.execute_command(Command::from_string(&input));
                input = String::new();
            }
            Err(error) => println!("Error reading input: {error}"),
        };
    }
}
