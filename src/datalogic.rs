use std::fs::{self, File, OpenOptions};
use std::io::Write;

#[allow(dead_code)]

pub fn writeln_to_file(line: &str) -> std::io::Result<()> {
    let mut f = File::options().append(true).create(true).open("data.txt")?;
    writeln!(&mut f, "{}", line)?;
    Ok(())
}

#[allow(dead_code)]
pub fn print_file() -> std::io::Result<()> {
    let content = fs::read_to_string("data.txt")?;
    let lines: Vec<&str> = content.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        println!("{}: {}", index + 1, line);
    }
    Ok(())
}

#[allow(dead_code)]
pub fn clear_file() -> std::io::Result<()> {
    let file = File::options().write(true).create(true).open("data.txt")?;
    file.set_len(0)?;
    Ok(())
}

#[allow(dead_code)]
pub fn delete_line(line_number: usize) -> std::io::Result<()> {
    // Read the whole file into a string
    let content = fs::read_to_string("data.txt")?;

    // Collect all lines into a Vec
    let mut lines: Vec<&str> = content.lines().collect();

    // Line numbers are usually 1-based, so check carefully
    if line_number == 0 || line_number > lines.len() {
        println!("Invalid Number! Run ls command to know the numbers of each task.");
    }

    // Remove the line (line_number - 1 because Vec is 0-based)
    lines.remove(line_number - 1);
    // Join the lines back into a single string with newlines
    let new_content = lines.join("\n");

    // Overwrite the file with the new content
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // clears the file
        .open("data.txt")?;
    file.write_all(new_content.as_bytes())?;
    writeln_to_file("");

    Ok(())
}
