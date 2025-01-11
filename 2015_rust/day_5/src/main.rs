use std::fs;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("2015_day_5_input.txt")?;
    let mut nice_string_counter = 0;

    for line in contents.lines() {
        /*
        if has_three_vowels(&line)
            && has_double_letter(&line)
            && does_not_have_defined_str(&line)
            {
                nice_string_counter += 1;
            }
            */

        if has_two_nonoverlapping_pairs(&line)
            && has_one_letter_in_between(&line)
            {
                nice_string_counter += 1;
            }
    }

    println!("# of nice strings: {}", nice_string_counter);

    Ok(())
}

/*
fn has_three_vowels(line: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    line.chars().filter(|c| vowels.contains(c)).take(3).count() == 3
}

fn has_double_letter(line: &str) -> bool {
    line.chars().tuple_windows().any(|(a, b)| a == b)
}

fn does_not_have_defined_str(line: &str) -> bool {
    !line.contains("ab") && !line.contains("cd") && !line.contains("pq") && !line.contains("xy")
}

*/

fn has_two_nonoverlapping_pairs(line: &str) -> bool {
    let mut pairs_map = HashMap::new();

    let bytes = line.as_bytes();

    for i in 0..bytes.len().saturating_sub(1) {
        let pair = (bytes[i], bytes[i + 1]);

        if let Some(&old_i) = pairs_map.get(&pair) {
            if i >= old_i + 2 {
                return true;
            }
        } else {
            pairs_map.insert(pair, i);
        }
    }

    false
}


fn has_one_letter_in_between(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();

    for i in 0..chars.len().saturating_sub(2) {
        if chars[i] == chars[i+2] {
            return true;
        }
    }

    return false
}
