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
        // Commands
        let mut parts = input.splitn(2, ' ');
        let command = parts.next().unwrap();
        let args = parts.next().unwrap_or(""); // rest of the words

        // comparing command
        match command {
            "exit" => break,
            "list" => datalogic::print_file(),
            "add" => datalogic::writeln_to_file(args),
            "remove" => datalogic::delete_line(convert_str_to_usize(args)),
            "clear" => datalogic::clear_file(),
            _ => datalogic::writeln_to_file(input),
        }
        .expect("Operation failed successfully")
    }
}
fn convert_str_to_usize(input: &str) -> usize {
    match input.parse::<usize>() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Error parsing string to usize: {}", e);
            1000
        }
    }
}
