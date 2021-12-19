use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE: &str = "input.txt";
// const FILE: &str = "test1.txt";

const DEBUG: bool = false;

fn main() -> Result<(), std::io::Error> {
    {
        // Part 1
        let f = File::open(FILE).unwrap();
        let f = BufReader::new(f);
        let mut horizontal = 0;
        let mut depth = 0;
        for line in f.lines() {
            let line = line?;
            let splits = line.split(' ').collect::<Vec<_>>();
            let direction = splits[0];
            let length = splits[1].parse::<usize>().unwrap();
            match direction {
                "forward" => {
                    horizontal += length;
                }
                "down" => {
                    depth += length;
                }
                "up" => {
                    depth -= length;
                }
                _ => {}
            }
        }
        println!("Part 1: Ans is: {}", horizontal * depth);
    }

    {
        // Part 2
        let f = File::open(FILE).unwrap();
        let f = BufReader::new(f);
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;
        for line in f.lines() {
            let line = line?;
            let splits = line.split(' ').collect::<Vec<_>>();
            let direction = splits[0];
            let length = splits[1].parse::<usize>().unwrap();
            match direction {
                "forward" => {
                    horizontal += length;
                    depth += aim * length;
                }
                "down" => {
                    aim += length;
                }
                "up" => {
                    aim -= length;
                }
                _ => {}
            }
            if DEBUG {
                println!("Line {:?}:", line);
                println!("\t{:?}", (horizontal, depth, aim));
                println!("\t{:?}", horizontal * depth);
            }
        }
        println!("Part 2: Ans is: {}", horizontal * depth);
    }

    Ok(())
}
