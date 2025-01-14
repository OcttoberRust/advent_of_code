use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("2015_day_7_input.txt")?;

    for line in contents.lines() {
        println!("{line}");
    }

    Ok(())
}
