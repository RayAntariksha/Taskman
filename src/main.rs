use colored::Colorize;
use std::io::{self, Write};

mod datalogic;

fn main() {
    loop {
        print!("{}", "[TASKMAN]# ".green());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        match input {
            "exit" => break,
            "list" => datalogic::print_file(),
            _ => datalogic::writeln_to_file(input),
        }
        .expect("Operation failed successfully")
    }
}
