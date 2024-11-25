use std::fs;
use std::cmp;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("dimensions.txt")?;
    let mut total_wrapping_paper = vec![];
    let mut total_ribbon = vec![];

    for line in contents.lines() {
        let v: Vec<&str> = line.split_terminator('x').collect();

        let present_box = Right_Rectangular_Prism::new(v);

        total_wrapping_paper.push(present_box.surface_area + present_box.smallest_side);
        total_ribbon.push(present_box.smallest_perimeter + present_box.cubic_volume);
    }

    let wrapping_paper_sum: u32 = total_wrapping_paper.iter().sum();
    let total_ribbon_sum: u32 = total_ribbon.iter().sum();

    println!("The wrapping paper sum is {wrapping_paper_sum} and the total_ribbon_sum is {total_ribbon_sum}");

    Ok(())
}

//box is a reserved keyword in Rust
struct Right_Rectangular_Prism {
    length: u32,
    width: u32,
    height: u32,
    lw: u32,
    wh: u32,
    hl: u32,
    surface_area: u32,
    smallest_side: u32,
    smallest_perimeter: u32,
    cubic_volume: u32,
}

impl Right_Rectangular_Prism {
    fn new(values: Vec<&str>) -> Self {
        let length = String::from(values[0]).parse().unwrap();
        let width = String::from(values[1]).parse().unwrap();
        let height = String::from(values[2]).parse().unwrap();

        let lw = length * width;
        let wh = width * height;
        let hl = height * length;

        Self {
            length,
            width,
            height,
            lw,
            wh,
            hl,
            surface_area: (2*lw) + (2*wh) + (2*hl),
            smallest_side: cmp::min(cmp::min(lw, wh), hl),
            smallest_perimeter: cmp::min(
            cmp::min(2*length+2*width, 2*width+2*height), 2*height+2*length),
            cubic_volume: length*width*height,
        }
    }
}
