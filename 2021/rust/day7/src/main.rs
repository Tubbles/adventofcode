use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(_lines: &Vec<String>) -> usize {
    let mut init_pos = _lines[0]
        .split(",")
        .into_iter()
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    init_pos.sort();
    println!("len = {}", init_pos.len());

    println!(
        "{}: {}",
        init_pos.len() / 2 - 1,
        init_pos[init_pos.len() / 2 - 1]
    );
    println!("{}: {}", init_pos.len() / 2, init_pos[init_pos.len() / 2]);
    println!(
        "{}: {}",
        init_pos.len() / 2 + 1,
        init_pos[init_pos.len() / 2 + 1]
    );

    init_pos[init_pos.len() / 2]
}

fn part2(_lines: &Vec<String>) -> usize {
    0
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
