use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    {
        // Part 1
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut num_valids = 0;
        for line in f.lines() {
            let mut input_splits = Vec::new();
            let line = line.expect("Unable to read line");
            for s in line.split(' ') {
                input_splits.push(s.trim());
            }
            // 14-15 h: hhhhhhhhhhhhhjh
            let bounds: Vec<&str> = input_splits[0].split('-').collect();
            let low_bound = bounds[0].parse::<i32>().expect("Unable to parse line");
            let high_bound = bounds[1].parse::<i32>().expect("Unable to parse line");
            let range = low_bound..high_bound + 1;
            let char = input_splits[1].chars().nth(0).unwrap();
            let pw = input_splits[2];
            let mut occurrences = 0;
            for c in pw.chars() {
                if c == char {
                    occurrences += 1;
                }
            }
            if range.contains(&occurrences) {
                num_valids += 1;
            }
        }
        println!("Part 1: Num valids: {}", num_valids);
    }

    {
        // Part 2
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut num_valids = 0;
        for line in f.lines() {
            let mut input_splits = Vec::new();
            let line = line.expect("Unable to read line");
            for s in line.split(' ') {
                input_splits.push(s.trim());
            }
            // 14-15 h: hhhhhhhhhhhhhjh
            let positions: Vec<&str> = input_splits[0].split('-').collect();
            let low_pos = positions[0].parse::<usize>().expect("Unable to parse line");
            let high_pos = positions[1].parse::<usize>().expect("Unable to parse line");

            let char = input_splits[1].chars().nth(0).unwrap();
            let pw = input_splits[2];

            if (pw.chars().nth(low_pos - 1).unwrap() == char)
                ^ (pw.chars().nth(high_pos - 1).unwrap() == char)
            {
                num_valids += 1;
            }
        }
        println!("Part 1: Num valids: {}", num_valids);
    }
}
