use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    {
        // Part 1
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut sum = 0;
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            // println!("Line {}: {}", i, line);
            let module_mass = line.parse::<i32>().expect("Unable to parse line");
            let idiv = (module_mass / 3) - 2;
            sum += idiv;
        }
        println!("Part 1: Sum is: {}", sum);
    }
    {
        // Part 2
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut sum = 0;
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            // println!("Line {}: {}", i, line);
            let module_mass = line.parse::<i32>().expect("Unable to parse line");
            let mut idiv = (module_mass / 3) - 2;
            loop
            {
                sum += idiv;
                idiv = (idiv / 3) - 2;
                if idiv <= 0 { break; }
            }
        }
        println!("Part 2: Sum is: {}", sum);
    }
}
