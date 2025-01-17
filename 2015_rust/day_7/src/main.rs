use std::fs;
use std::io;

enum Instruction {
    Value(u16),
    Wire(String),
    And(String, String),
    Or(String, String),
    Not(String),
    LShift(String, u16),
    RShift(String, u16),
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
