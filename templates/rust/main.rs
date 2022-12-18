use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    {
        // Asserts
    }
    {
        // Part 1
        for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
            let _line = line.unwrap();
        }
        println!("Part 1: Ans is: {}", 0);
    }
    {
        // Part 2
        for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
            let _line = line.unwrap();
        }
        println!("Part 2: Ans is: {}", 0);
    }
}
