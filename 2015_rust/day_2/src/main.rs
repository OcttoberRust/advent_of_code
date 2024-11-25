use std::fs;
use std::cmp;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("dimensions.txt")?;
    let mut total_wrapping_paper = vec![];

    for line in contents.lines() {
        println!("{line}");

        let v: Vec<&str> = line.split_terminator('x').collect();

        println!("{:?}", v);

        let length: u32 = String::from(v[0]).parse().unwrap();
        let width: u32 = String::from(v[1]).parse().unwrap();
        let height: u32 = String::from(v[2]).parse().unwrap();
         //^maybe use a tuple here?

        let side1 = length*width;
        let side2 = width*height;
        let side3 = height*length;
         
        let surface_area = (2*side1) + (2*side2) + (2*side3);

        let min = cmp::min(cmp::min(side1, side2), side3);
        //in the statements above this comment, are the variables being borrowed? 

        total_wrapping_paper.push(surface_area + min);

    }

    let sum: u32 = total_wrapping_paper.iter().sum();

    println!("The sum is: {sum}");

    Ok(())
}
