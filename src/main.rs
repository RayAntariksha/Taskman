use colored::Colorize;
use std::fs::File;
use std::io::{self, Write};

mod datalogic;

fn main() -> std::io::Result<()> {
    File::options().write(true).create(true).open("data.txt")?;
    loop {
        //This is the prompt
        print!("{}", "[TASKMAN]# ".green());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        //Taking input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: &str = input.trim();
        // Commands
        let mut parts = input.splitn(2, ' ');
        let command = parts.next().unwrap();
        let args = parts.next().unwrap_or(""); // rest of the words

        // comparing command
        match command {
            "exit" => break,
            "ls" => datalogic::print_file(),
            "add" => datalogic::writeln_to_file(args),
            "rm" => datalogic::delete_line(convert_str_to_usize(args)),
            "rm-all" => datalogic::clear_file(),
            "md" => {
                if args.is_empty() {
                    println!("Please provide a task description.");
                    Ok(())
                } else {
                    let _ = datalogic::mark_done(convert_str_to_usize(args)); // Assuming mark_done is defined in datalogic.rs
                    Ok(())
                }
            }
            "clear" => Ok({
                // The line below clears the terminal screen
                print!("\x1B[2J\x1B[1;1H");
            }),
            _ => Ok(println!("Invalid command")),
        }
        .expect("Operation failed successfully")
    }
    Ok(())
}
// A simple function to convert string to number
fn convert_str_to_usize(input: &str) -> usize {
    match input.parse::<usize>() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Error parsing string to usize: {}", e);
            1000
        }
    }
}
