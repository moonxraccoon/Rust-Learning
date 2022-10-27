pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }
}

pub struct Calculator {
    numbers: Stack<i64>,
}

pub enum Command {
    Push(i64),
    Pop,
    Add,
    Sub,
    Multiply,
    Divide,
    IfElse,
    Print,
    Revert,
    Reset,
    Quit,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            numbers: Stack::new(),
        }
    }
    pub fn execute_command(&mut self, cmd: Command) {
        cmd.call(self);
    }
}

impl Command {
    pub fn from_string(str: &String) -> Self {
        let mut iter = str.split_whitespace();
        let cmd = iter.next().unwrap_or("quit");
        let arg: i64 = iter.next().unwrap_or("0").parse().unwrap();
        match cmd {
            "push" => Command::Push(arg),
            "pop" => Command::Pop,
            "add" => Command::Add,
            "sub" => Command::Sub,
            "multiply" => Command::Multiply,
            "divide" => Command::Divide,
            "if-else" => Command::IfElse,
            "print" => Command::Print,
            "revert" => Command::Revert,
            "reset" => Command::Reset,
            "quit" => Command::Quit,
            &_ => Command::Quit,
        }
    }
    pub fn call(self, calc: &mut Calculator) {
        match self {
            Command::Pop => {
                calc.numbers.pop();
                println!("Ok!");
            }
            Command::Add => {
                if calc.numbers.size() < 2 {
                    println!("There are not enough numbers!");
                    return;
                }
                let a = calc.numbers.pop().unwrap();
                let b = calc.numbers.pop().unwrap();
                calc.numbers.push(a + b);
                println!("Ok!");
            }
            Command::Push(num) => {
                calc.numbers.push(num);
                println!("Ok!");
            }
            Command::Sub => {
                if calc.numbers.size() < 2 {
                    println!("There are not enough numbers!");
                    return;
                }
                let a = calc.numbers.pop().unwrap();
                let b = calc.numbers.pop().unwrap();
                calc.numbers.push(a - b);
                println!("Ok!");
            }
            Command::Multiply => {
                if calc.numbers.size() < 2 {
                    println!("There are not enough numbers!");
                    return;
                }
                let a = calc.numbers.pop().unwrap();
                let b = calc.numbers.pop().unwrap();
                calc.numbers.push(a * b);
                println!("Ok!");
            }
            Command::Divide => {
                if calc.numbers.size() < 2 {
                    println!("There are not enough numbers!");
                    return;
                }
                let a = calc.numbers.pop().unwrap();
                let b = calc.numbers.pop().unwrap();
                calc.numbers.push(a / b);
                println!("Ok!");
            }
            Command::IfElse => {
                println!("If-Else");
            }
            Command::Print => {
                if calc.numbers.stack.is_empty() {
                    println!("Stack is empty!");
                    return;
                }
                print!("|");
                for i in calc.numbers.stack.iter().rev() {
                    print!(" {} |", i);
                }
                println!("");
            }
            Command::Revert => {
                let mut new_nums: Stack<i64> = Stack::new();
                while calc.numbers.size() > 0 {
                    new_nums.push(calc.numbers.pop().unwrap_or(0));
                }
                calc.numbers = new_nums;
                println!("Ok!");
            }
            Command::Reset => {
                calc.numbers = Stack::new();
                println!("Ok!");
            }
            Command::Quit => {}
        };
    }
}
