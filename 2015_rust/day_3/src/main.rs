use std::fs;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut contents = fs::read_to_string("santa_directions.txt")?;
    let mut unique_houses = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    unique_houses.insert((x, y));

    for character in contents.chars() {
        println!("{character}");

        match character {
            '^' => x += 1,
            'v' => x -= 1,
            '<' => y -= 1,
            '>' => y += 1,
            _ => (),
        }

        unique_houses.insert((x, y));
    }

    println!("{:?}", unique_houses);
    println!("{}", unique_houses.len());

    Ok(())
}
