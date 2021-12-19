use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_binary_positions(lines: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut zeroes = Vec::new();
    let mut ones = Vec::new();

    for _ in 0..lines[0].len() {
        zeroes.push(0);
        ones.push(0);
    }

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                zeroes[i] += 1;
            } else {
                ones[i] += 1;
            }
        }
    }

    (zeroes, ones)
}

fn binary_diagsnotic(zeroes: &Vec<usize>, ones: &Vec<usize>) -> (usize, usize) {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in 0..zeroes.len() {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if ones[i] > zeroes[i] {
            gamma_rate += 1;
        } else {
            epsilon_rate += 1;
        }
    }

    (gamma_rate, epsilon_rate)
}

const OXYGEN_GENERATOR_RATING: fn(usize, usize) -> char = |x, y| if x > y { '0' } else { '1' };
const CO2_SCRUBBER_RATING: fn(usize, usize) -> char = |x, y| if x > y { '1' } else { '0' };

fn life_support_rating(lines: &Vec<String>, lambda: fn(usize, usize) -> char) -> Option<usize> {
    let mut lines = lines.clone();
    for i in 0..lines[0].len() {
        let (zeroes, ones) = count_binary_positions(&lines);
        let matcher = lambda(zeroes[i], ones[i]);
        lines = lines
            .into_iter()
            .filter(|s| s.chars().collect::<Vec<_>>()[i] == matcher)
            .collect();
        if lines.len() == 1 {
            return Some(usize::from_str_radix(lines[0].as_str(), 2).unwrap());
        }
    }
    None
}

fn part1(lines: &Vec<String>) -> usize {
    let (zeroes, ones) = count_binary_positions(&lines);
    let (gamma_rate, epsilon_rate) = binary_diagsnotic(&zeroes, &ones);
    gamma_rate * epsilon_rate
}

fn part2(lines: &Vec<String>) -> usize {
    let oxygen = life_support_rating(&lines, OXYGEN_GENERATOR_RATING).unwrap();
    let co2 = life_support_rating(&lines, CO2_SCRUBBER_RATING).unwrap();
    oxygen * co2
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
