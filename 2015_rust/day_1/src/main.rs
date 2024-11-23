use std::fs;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut contents = fs::read_to_string("advent_of_code_day_1_text_input.txt")?;
    
    let mut counter = 0;
    
    for character in contents.chars() {
        match character {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => (),
        }
    }

    println!("counter: {counter}");
    Ok(())
}
