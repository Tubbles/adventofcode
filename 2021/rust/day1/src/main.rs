use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::VecDeque;

const WINDOW_SIZE: usize = 3;

fn main() {
    {
        // Part 1
        let f = File::open("input.txt").unwrap();
        let f = BufReader::new(f);
        let mut ans = 0;
        let mut prev = 0;
        for line in f.lines() {
            let line = line.unwrap();
            let entry = line.parse::<i32>().unwrap();
            if prev != 0 && entry > prev {
                ans += 1;
            }
            prev = entry;
        }
        println!("Part 1: Ans is: {}", ans);
    }

    {
        // Part 2
        let f = File::open("input.txt").unwrap();
        let f = BufReader::new(f);
        let mut ans = 0;
        let mut prev = None;
        let mut buf = VecDeque::with_capacity(WINDOW_SIZE as usize);
        for line in f.lines() {
            let line = line.unwrap();
            let entry = line.parse::<usize>().unwrap();
            buf.push_back(entry);
            if buf.len() == WINDOW_SIZE as usize {
                let avg = buf.iter().sum::<usize>() as f32 / WINDOW_SIZE as f32;
                buf.pop_front();
                if prev.is_some() && avg > prev.unwrap() {
                    ans += 1;
                }
                prev = Some(avg);
            }
        }
        println!("Part 2: Ans is: {}", ans);
    }
}
