use std::fs;
use std::io;

enum Instruction {
    //need to implement
}

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("2015_day_7_input.txt")?;

    for line in contents.lines() {
        println!("{line}");
        if line.trim().is_empty() {
            continue;
        }
    }

    Ok(())
}

fn parse_line(line: &str) {

}

fn get_signal() {

}
