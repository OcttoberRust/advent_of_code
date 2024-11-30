use std::fs;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("santa_directions.txt")?;
    let mut unique_houses: HashSet<(i32, i32)> = HashSet::new();
    let mut santa_coords = Coordinates { x: 0, y: 0 };
    let mut robo_santa_coords = Coordinates { x: 0, y: 0 };

    unique_houses.insert((0, 0));

    let directions: Vec<char> = contents.chars().collect();
    for (i, &direction) in directions.iter().enumerate() {
        if i % 2 == 0 {
            santa_coords.update(direction);
            unique_houses.insert((santa_coords.x, santa_coords.y));
        } else {
            robo_santa_coords.update(direction);
            unique_houses.insert((robo_santa_coords.x, robo_santa_coords.y));
        }
    }

    println!("Unique houses visited: {}", unique_houses.len());

    Ok(())
}

struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn update(&mut self, direction: char) {
        match direction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            '>' => self.x += 1,
            _ => (),
        }
    }
}
