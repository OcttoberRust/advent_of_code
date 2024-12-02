use std::fs;

fn main() {
    let contents = fs::read_to_string("2015_day_5_input.txt");

    for line in contents {
        println!("{line}");
    }
}
