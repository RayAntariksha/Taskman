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
