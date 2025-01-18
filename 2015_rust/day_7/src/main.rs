use std::fs;
use std::io;
use std::collections::HashMap;

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

fn parse_line(line: &str) -> (String, Instruction) {

    (String::from("dest_wire"), Instruction::Value(0))
}

fn get_signal(wire: &str, circuit: &HashMap<String, Instruction>, memo: &mut HashMap<String, u16>)
-> u16 {

    0
}
