use std::fs;
use std::io;

const GRID_SIZE: usize = 1000;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("2015_day_6_input.txt")?;

    let mut grid = vec![0u32; GRID_SIZE * GRID_SIZE];

    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let instruction = Instruction::from_line(trimmed);

        update_brightness(&mut grid, &instruction);
    }

    let total_brightness: u32 = grid.iter().sum();
    println!("Total brightness: {}", total_brightness);

    Ok(())
}

fn update_brightness(grid: &mut [u32], instr: &Instruction) {
    for row in instr.y1..=instr.y2 {
        for col in instr.x1..=instr.x2 {
            let idx = row * GRID_SIZE + col;
            match instr.op {
                Operation::TurnOn => grid[idx] += 1,
                Operation::TurnOff => {
                    if grid[idx] > 0 {
                        grid[idx] -= 1;
                    }
                }
                Operation::Toggle => grid[idx] += 2,
            }
        }
    }
}

enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    op: Operation,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let line = line.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        println!("Parsing line: {:?}", parts);

        let (op, coord_start, coord_end) = if parts[0] == "toggle" {
            if parts.len() < 4 {
                panic!("Unexpected format: {}", line);
            }
            (Operation::Toggle, parts[1], parts[3])
        } else {
            if parts.len() < 5 {
                panic!("Unexpected format: {}", line);
            }
            let op = match parts[1] {
                "on" => Operation::TurnOn,
                "off" => Operation::TurnOff,
                _ => panic!("Unknown operation in line: {}", line),
            };
            (op, parts[2], parts[4])
        };

        let parse_pair = |s: &str| -> (usize, usize) {
            let nums: Vec<usize> = s
            .split(',')
            .map(|s| s.trim())
            .map(|s| s.parse().expect(&format!("invalid number in '{}'", s)))
            .collect();
            if nums.len() != 2 {
                panic!("Expected two numbers in the coordinate but got {:?}", nums);
            }
            (nums[0], nums[1])
        };

        let (x1, y1) = parse_pair(coord_start);
        let (x2, y2) = parse_pair(coord_end);

        Self { op, x1, y1, x2, y2 }
    }
}


