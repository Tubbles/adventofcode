use std::fs::File;
use std::io::{BufRead, BufReader};

const SPAWN_RATE: usize = 7;
const BABY_DELAY: usize = 2;
const SIM_TICKS_PART1: usize = 80;
const SIM_TICKS_PART2: usize = 256;

struct Lanternfish {
    days_left: usize,
}

impl Lanternfish {
    fn new() -> Lanternfish {
        Lanternfish {
            days_left: SPAWN_RATE + BABY_DELAY - 1,
        }
    }

    fn from_ticker(ticker: usize) -> Lanternfish {
        Lanternfish { days_left: ticker }
    }

    fn tick(&mut self) -> bool {
        if self.days_left > 0 {
            self.days_left -= 1;
        } else {
            self.days_left = SPAWN_RATE - 1;
            return true;
        }
        false
    }
}

fn part1(lines: &Vec<String>) -> usize {
    let mut fishies = Vec::new();
    for num in lines[0].split(",").collect::<Vec<_>>() {
        fishies.push(Lanternfish::from_ticker(num.parse().unwrap()));
    }

    for _ in 0..SIM_TICKS_PART1 {
        let mut babies = Vec::new();
        for fish in &mut fishies {
            if fish.tick() {
                babies.push(Lanternfish::new());
            }
        }
        fishies.append(&mut babies);
    }

    fishies.len()
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut bins = vec![0u64; SPAWN_RATE + BABY_DELAY];
    for num in lines[0].split(",").collect::<Vec<_>>() {
        bins[num.parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..SIM_TICKS_PART2 {
        let new_babies = bins[0];
        for i in 0..(bins.len() - 1) {
            bins[i] = bins[i + 1];
        }
        bins[SPAWN_RATE - 1] += new_babies;
        bins[SPAWN_RATE + BABY_DELAY - 1] = new_babies;
        // println!("{:?}", bins);
    }

    bins.iter().sum()
}

fn main() -> Result<(), std::io::Error> {
    let f = BufReader::new(File::open("test1.txt").unwrap());
    let lines = f.lines().filter_map(|s| s.ok()).collect::<Vec<_>>();
    println!("Test Part 1: Ans is: {}", part1(&lines));
    println!("Test Part 2: Ans is: {}", part2(&lines));

    let f = BufReader::new(File::open("input.txt").unwrap());
    let lines = f.lines().filter_map(|s| s.ok()).collect::<Vec<_>>();
    println!("Part 1: Ans is: {}", part1(&lines));
    println!("Part 2: Ans is: {}", part2(&lines));

    Ok(())
}
