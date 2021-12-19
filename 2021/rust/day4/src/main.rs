use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const TOTAL_NUMS: usize = WIDTH * HEIGHT;
const NUM_DIGITS: usize = 2;

struct Player {
    board: [(usize, bool); TOTAL_NUMS],
}

impl fmt::Debug for Player {
    fn fmt(self: &Player, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("Player, board = [")?;
        for y in 0..HEIGHT {
            fmt.write_str("\n\t")?;
            for x in 0..WIDTH {
                let mark_start = if self.board[x + y * WIDTH].1 {
                    '('
                } else {
                    ' '
                };
                let mark_end = if self.board[x + y * WIDTH].1 {
                    ')'
                } else {
                    ' '
                };
                fmt.write_fmt(format_args!(
                    "{}{:width$}{}",
                    mark_start,
                    self.board[x + y * WIDTH].0,
                    mark_end,
                    width = NUM_DIGITS
                ))?;
            }
        }
        fmt.write_str("\n]")
    }
}

impl Player {
    fn from_lines(lines: &[String]) -> Player {
        let mut p = Player {
            board: [(0, false); TOTAL_NUMS],
        };
        for y in 0..HEIGHT {
            // println!("{}", lines[y]);
            let nums = lines[y]
                .split(' ')
                .filter_map(|s| s.parse::<usize>().map_or(None, |s| Some(s)))
                .collect::<Vec<_>>();
            for x in 0..WIDTH {
                p.board[x + y * WIDTH] = (nums[x], false);
            }
        }
        p
    }

    fn is_column_complete(&self, column_num: usize) -> bool {
        for y in 0..HEIGHT {
            if !self.board[column_num + y * WIDTH].1 {
                return false;
            }
        }
        true
    }

    fn is_row_complete(&self, row_num: usize) -> bool {
        for x in 0..WIDTH {
            if !self.board[x + row_num * WIDTH].1 {
                return false;
            }
        }
        true
    }

    fn get_sum_unmarked(&self) -> usize {
        self.board
            .iter()
            .filter_map(|x| if !x.1 { Some(x.0) } else { None })
            .sum()
    }

    // Check a new number on the board
    // If the board wins (5 in a row), the return value is Some(<score>),
    // else it returns None
    fn add_number(&mut self, num: usize) -> Option<usize> {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.board[x + y * WIDTH].0 == num {
                    self.board[x + y * WIDTH].1 = true;

                    if self.is_column_complete(x) || self.is_row_complete(y) {
                        return Some(self.get_sum_unmarked() * num);
                    }

                    break;
                }
            }
        }
        None
    }
}

fn part1(lines: &Vec<String>) -> usize {
    let numbers = lines[0]
        .split(',')
        .filter_map(|s| s.parse::<usize>().map_or(None, |s| Some(s)))
        .collect::<Vec<_>>();
    // println!("{:?}", numbers);

    let mut players = Vec::new();
    for i in 0..((lines.len() - 1) / (HEIGHT + 1)) {
        let from_index = i * (HEIGHT + 1) + 2;
        players.push(Player::from_lines(&lines[from_index..from_index + WIDTH]));
    }

    let mut winner = None;
    let mut counter = 0;
    while winner.is_none() {
        for (_i, p) in &mut players.iter_mut().enumerate() {
            winner = p.add_number(numbers[counter]);
            // println!("p[{}] = {:?}\n", i, p);
            if winner.is_some() {
                break;
            }
        }
        counter += 1;
    }
    winner.unwrap()
}

fn part2(lines: &Vec<String>) -> usize {
    let numbers = lines[0]
        .split(',')
        .filter_map(|s| s.parse::<usize>().map_or(None, |s| Some(s)))
        .collect::<Vec<_>>();
    // println!("{:?}", numbers);

    let mut players = Vec::new();
    for i in 0..((lines.len() - 1) / (HEIGHT + 1)) {
        let from_index = i * (HEIGHT + 1) + 2;
        players.push((
            Player::from_lines(&lines[from_index..from_index + WIDTH]),
            false,
        ));
    }

    let mut winner = None;
    let mut counter = 0;
    let mut still_playing = true;
    while still_playing {
        still_playing = false;
        for (_i, p) in &mut players.iter_mut().enumerate() {
            if !p.1 {
                still_playing = true;
                winner = p.0.add_number(numbers[counter]);
                // println!("p[{}] = {:?}\n", _i, p);
                if winner.is_some() {
                    // println!("p[{}] winner ! {}", _i, winner.unwrap());
                    p.1 = true;
                }
            }
        }
        counter += 1;
    }
    winner.unwrap()
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
