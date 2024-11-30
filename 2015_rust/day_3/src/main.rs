use std::fs;
use std::str::Chars;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut contents = fs::read_to_string("santa_directions.txt")?;
    let mut unique_houses: HashSet<(i32, i32)> = HashSet::new();
    let mut santa_coords = Coordinates { x: 0, y: 0 };
    let mut robo_santa_coords = Coordinates { x: 0, y: 0 };
    let mut iteration = 0;
    let (mut x, mut y) = (0, 0); 

    unique_houses.insert((0, 0));

    for character in contents.chars() {

        if(iteration % 2 == 0) {
            (x, y) =  track_coords(character, &mut santa_coords);
            unique_houses.insert((x, y));
        } else {
            (x, y) = track_coords(character, &mut robo_santa_coords);
            unique_houses.insert((x, y));
        }
        
        iteration += 1;
    }

    println!("{}", unique_houses.len());

    Ok(())
}

struct Coordinates {
    x: i32,
    y: i32,
}

fn track_coords(character: char, coords: &mut Coordinates) -> (i32, i32) {
    match character {
        '^' => coords.x += 1,
        'v' => coords.x -= 1,
        '<' => coords.y -= 1,
        '>' => coords.y += 1,
        _ => (),
    }

    (coords.x, coords.y) 
}

