use std::fs;
use std::thread;
use std::sync::Arc;
use md5;

fn main() -> std::io::Result<()> {
    let contents = Arc::new(fs::read_to_string("2015_day_4_input.txt")?.trim().to_string());

    let mut handles = vec![];

    let ranges = vec![
        (1,        1000000),
        (1000001,   2000000),
        (2000001,   3000000),
        (3000001,   4000000),
        (4000001,   5000000),
        (5000001,   6000000),
        (6000001,   7000000),
        (7000001,   8000000),
    ];

    println!("Starting computation...");

    for (start, end) in ranges {
        let data_clone = Arc::clone(&contents);
        let handle = thread::spawn(move || {
            for i in start..=end {
                let input = format!("{}{}", data_clone, i);

                let digest = md5::compute(input.as_bytes());
                let hash_str = format!("{:x}", digest);

                if hash_str.starts_with("000000") {
                    println!("i = {}", i);
                    println!("The matching hash is: {}", hash_str);
                    break;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Computation completed.");

    Ok(())
}
