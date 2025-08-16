use std::fs::File;
use std::io::{Read, Write};
pub fn writeln_to_file(line: &str) -> std::io::Result<()> {
    let mut f = File::options().append(true).create(true).open("data.txt")?;
    writeln!(&mut f, "{}", line)?;
    Ok(())
}
pub fn print_file() -> std::io::Result<()> {
    let mut file = File::open("data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
pub fn clear_file() -> std::io::Result<()> {
    let mut file = File::options().write(true).create(true).open("data.txt")?;
    file.set_len(0)?;
    Ok(())
}
pub fn delete_line(line_number: usize) -> std::io::Result<()> {
    let mut file = File::options().write(true).create(true).open("data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.lines().collect();
    if line_number > lines.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Line number out of range",
        ));
    }
    let mut new_contents = String::new();
    for (i, line) in lines.iter().enumerate() {
        if i != line_number - 1 {
            new_contents.push_str(line);
            new_contents.push('\n');
        }
    }
    file.set_len(0)?;
    file.write_all(new_contents.as_bytes())?;
    Ok(())
}
