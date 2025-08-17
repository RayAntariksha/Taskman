use std::fs::{self, File, OpenOptions};
use std::io::Write;

#[allow(dead_code)]

pub fn writeln_to_file(line: &str) -> std::io::Result<()> {
    let mut f = File::options().append(true).create(true).open("data.txt")?;
    writeln!(&mut f, "{}", line)?;
    writeln!(&mut f, "[]")?;
    Ok(())
}

#[allow(dead_code)]

pub fn print_file() -> std::io::Result<()> {
    let content = fs::read_to_string("data.txt")?;
    let lines: Vec<&str> = content.lines().collect();
    let mut num = 1;
    for (index, line) in lines.iter().enumerate() {
        if index % 2 == 0 {
            println!("{}: {}", num, line);
            num += 1; // Increment num for the next line
        } else {
            continue; // Skip every second line (the empty lines)
        }
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
#[allow(unused_must_use)]

pub fn delete_line(line_number: usize) -> std::io::Result<()> {
    // Read the whole file into a string
    let content = fs::read_to_string("data.txt")?;

    // Collect all lines into a Vec
    let mut lines: Vec<&str> = content.lines().collect();

    // Line numbers are usually 1-based, so check carefully
    if line_number == 0 || line_number > lines.len() {
        println!("Invalid Number! Run ls command to know the numbers of each task.");
        return Ok(());
    }
    let line_number = line_number* 2; // Adjust for the empty lines
    // Remove the line (line_number - 1 because Vec is 0-based)
    lines.remove(line_number - 1);
    lines.remove(line_number - 2); // Remove the corresponding empty line
    // Join the lines back into a single string with newlines
    let new_content = lines.join("\n");

    // Overwrite the file with the new content
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // clears the file
        .open("data.txt")?;
    file.write_all(new_content.as_bytes())?;
    writeln!(file, "")?; // Ensure the file ends with a newline

    Ok(())
}
