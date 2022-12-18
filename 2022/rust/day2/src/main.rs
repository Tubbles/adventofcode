use std::fs::File;
use std::io::{BufRead, BufReader};

// Rock     = A, X, 1 point
// Paper    = B, Y, 2 points
// Scissors = C, Z, 3 points
// 0 for loss, 3 for draw, 6 for win

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Debug)]
enum Result {
    Draw,
    Lose,
    Win,
}

fn wins_over(p: &Play) -> Play {
    match p {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock,
    }
}

fn loses_to(p: &Play) -> Play {
    match p {
        Play::Rock => Play::Scissors,
        Play::Paper => Play::Rock,
        Play::Scissors => Play::Paper,
    }
}

fn result(me: &Play, you: &Play) -> Result {
    if *me == *you {
        Result::Draw
    } else if *me == wins_over(you) {
        Result::Win
    } else {
        Result::Lose
    }
}

fn score(result: &Result) -> isize {
    match result {
        Result::Lose => 0,
        Result::Draw => 3,
        Result::Win => 6,
    }
}

fn str_to_play(s: &str) -> Play {
    match s {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        _ => Play::Scissors,
    }
}

fn to_points(p: &Play) -> isize {
    match p {
        Play::Rock => 1,
        Play::Paper => 2,
        _ => 3,
    }
}

fn parse_part1(s: &str) -> isize {
    let mut splits = s.split(" ");
    let you = str_to_play(splits.next().unwrap());
    let me = str_to_play(splits.next().unwrap());
    to_points(&me) + score(&result(&me, &you))
}

fn from_strategy(s: &str, other: &Play) -> Play {
    match s {
        "X" => loses_to(other),
        "Y" => *other,
        _ => wins_over(other),
    }
}

fn parse_part2(s: &str) -> isize {
    let mut splits = s.split(" ");
    let you = str_to_play(splits.next().unwrap());
    let me = from_strategy(splits.next().unwrap(), &you);
    to_points(&me) + score(&result(&me, &you))
}

fn main() {
    {
        // Asserts
        assert!(parse_part1("A X") == 4);
        assert!(parse_part1("B X") == 1);
        assert!(parse_part1("C X") == 7);
        assert!(parse_part1("A Y") == 8);
        assert!(parse_part1("B Y") == 5);
        assert!(parse_part1("C Y") == 2);
        assert!(parse_part1("A Z") == 3);
        assert!(parse_part1("B Z") == 9);
        assert!(parse_part1("C Z") == 6);
    }
    {
        // Part 1
        let f = File::open("input.txt").unwrap();
        let f = BufReader::new(f);
        let mut points = 0;
        for line in f.lines() {
            let line = line.unwrap();
            points += parse_part1(&line);
        }
        println!("Part 1: Ans is: {}", points);
    }
    {
        // Part 2
        let f = File::open("input.txt").unwrap();
        let f = BufReader::new(f);
        let mut points = 0;
        for line in f.lines() {
            let line = line.unwrap();
            points += parse_part2(&line);
        }
        println!("Part 2: Ans is: {}", points);
    }
}
