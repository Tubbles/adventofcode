use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

const CIPHER_SIZE: usize = 25;

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut vec = VecDeque::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let entry = line.parse::<usize>().expect("Unable to parse line");
        vec.push_back(entry);
    }
    let vec = vec;

    let mut part1 = 0;
    {
        // Part 1
        let mut vec1 = VecDeque::new();
        for entry in &vec {
            if vec1.len() == CIPHER_SIZE {
                let mut found = false;
                'outer: for (i, num1) in vec1.iter().enumerate() {
                    for (j, num2) in vec1.iter().enumerate() {
                        if (i != j) && (num1 + num2) == *entry {
                            found = true;
                            break 'outer;
                        }
                    }
                }
                if found == false {
                    part1 = *entry;
                    break;
                }
            }
            vec1.push_back(*entry);
            if vec1.len() > CIPHER_SIZE {
                vec1.pop_front();
            }
        }
        println!("Part 1: {}", part1);
    }
    let part1 = part1;
    {
        // Part 2
        let mut out: Vec<usize> = Vec::new();
        let mut found = false;
        for i1 in 0..vec.len() {
            out.clear();
            for num2 in vec.iter().skip(i1) {
                out.push(*num2);
                let len = out.len();
                let sum = out.iter().sum::<usize>(); // Why is turbo fish required??
                if len > 1 && sum == part1 {
                    found = true;
                    break;
                } else if sum > part1 {
                    break;
                }
            }
            if found {
                break;
            }
        }
        out.sort();
        println!("Part 2: {}", out.first().unwrap() + out.last().unwrap());
    }
}
