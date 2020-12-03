use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    {
        // Part 1
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut ans = 0;
        let mut vec = Vec::new();
        'outer1: for line in f.lines() {
            let line = line.expect("Unable to read line");
            let entry = line.parse::<i32>().expect("Unable to parse line");
            for e in &vec {
                if entry + e == 2020 {
                    ans = entry * e;
                    break 'outer1;
                }
            }
            vec.push(entry);
        }
        println!("Part 1: Ans is: {}", ans);
    }

    {
        // Part 2
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut ans = 0;
        let mut vec = Vec::new();
        'outer2: for line in f.lines() {
            let line = line.expect("Unable to read line");
            let entry = line.parse::<i32>().expect("Unable to parse line");
            for e1 in &vec {
                for e2 in &vec {
                    if entry + e1 + e2 == 2020 {
                        ans = entry * e1 * e2;
                        break 'outer2;
                    }
                }
            }
            vec.push(entry);
        }
        println!("Part 2: Ans is: {}", ans);
    }
}
