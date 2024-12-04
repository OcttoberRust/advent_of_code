use std::fs;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("2015_day_5_input.txt")?;
    let mut nice_string_counter = 0;

    for line in contents.lines() {
        println!("{line}");
        
        let mut criteria_met = 0;

        if has_three_vowels(&line) {
            criteria_met += 1;
        }

        println!("Criteria met: {}", criteria_met);
        
        if has_double_letter(&line) {
            criteria_met += 1;
        }

        println!("Criteria met: {}", criteria_met);

        if does_not_have_defined_str(&line) {
            criteria_met += 1;
        }

        println!("Criteria met: {}", criteria_met);
        
        if criteria_met == 3 {
            nice_string_counter += 1;
        }
    }

    println!("# of nice strings: {nice_string_counter}");

    Ok(())
}

fn has_three_vowels(line: &str) -> bool {

    let mut vowel_counter = 0;
    
    for char in line.chars() {
        println!("{char} ");
        match char {
            'a' => vowel_counter += 1,
            'e' => vowel_counter += 1,
            'i' => vowel_counter += 1,
            'o' => vowel_counter += 1,
            'u' => vowel_counter += 1,
             _  => (),
        }

        if vowel_counter == 3 {
            return true
        }
    }
    
    false
}

fn has_double_letter(line: &str) -> bool {

    let mut chars = line.chars().peekable();

    while let Some(current_char) = chars.next() {
        if let Some(&next_char) = chars.peek() {
            if current_char == next_char {
                return true;
            }
        }
    }

    false
}

fn does_not_have_defined_str(line: &str) -> bool {

    let re = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    if re.is_match(line) {
        return false;
    }

    true
}
