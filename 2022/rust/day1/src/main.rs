use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    {
        // Part 1
        let f = File::open("input.txt").unwrap();
        let f = BufReader::new(f);
        let mut cal_max = 0;
        let mut cal_cur = 0;
        for line in f.lines() {
            let line = line.unwrap();
            if line == "" {
                cal_max = max(cal_max, cal_cur);
                cal_cur = 0;
            } else {
                cal_cur += line.parse::<i32>().unwrap();
            }
        }
        println!("Part 1: Ans is: {}", cal_max);
    }
    {
        // Part 2
        let f = File::open("input.txt").unwrap();
        let f = BufReader::new(f);
        let mut vec = Vec::new();
        let mut cal_cur = 0;
        for line in f.lines() {
            let line = line.unwrap();
            if line == "" {
                vec.push(cal_cur);
                cal_cur = 0;
            } else {
                cal_cur += line.parse::<i32>().unwrap();
            }
        }
        vec.sort();
        vec.reverse();
        println!("Part 2: Ans is: {}", vec[0..3].iter().sum::<i32>());
    }
}
