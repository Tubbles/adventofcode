// #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug, serde::Serialize)]
// #[rustfmt::skip]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use num_bigint::BigInt;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug, serde::Serialize)]
enum Operation {
    #[default]
    None,
    Add(usize),
    Mult(usize),
    Power(usize),
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Debug, serde::Serialize)]
struct Monke {
    items: std::collections::VecDeque<BigInt>,
    op: Operation,
    test: usize,
    true_monke: usize,
    false_monke: usize,
    items_inspected: usize,
}

impl Monke {
    fn from_file(path: &str) -> Vec<Monke> {
        let mut monkes = Vec::new();

        let mut index = 0;

        for line in BufReader::new(fs::File::open(path).unwrap()).lines() {
            let line = line.unwrap();
            let mut int = 0;
            let mut string = String::new();

            //Monkey 0:
            //  Starting items: 79, 98
            //  Operation: new = old * 19
            //  Test: divisible by 23
            //    If true: throw to monkey 2
            //    If false: throw to monkey 3

            if scanf::sscanf!(&line, "Monkey {}:", int).is_ok() {
                index = int;
                monkes.push(Monke::default());
            } else if scanf::sscanf!(&line, "  Starting items: {}", string).is_ok() {
                monkes[index].items = string
                    .split(", ")
                    .map(|x| x.parse::<BigInt>().unwrap())
                    .collect();
            } else if scanf::sscanf!(&line, "  Operation: new = old + {}", int).is_ok() {
                monkes[index].op = Operation::Add(int);
            } else if scanf::sscanf!(&line, "  Operation: new = old * {}", int).is_ok() {
                monkes[index].op = Operation::Mult(int);
            } else if &line == "  Operation: new = old * old" {
                monkes[index].op = Operation::Power(2);
            } else if scanf::sscanf!(&line, "  Test: divisible by {}", int).is_ok() {
                monkes[index].test = int;
            } else if scanf::sscanf!(&line, "    If true: throw to monkey {}", int).is_ok() {
                monkes[index].true_monke = int;
            } else if scanf::sscanf!(&line, "    If false: throw to monkey {}", int).is_ok() {
                monkes[index].false_monke = int;
            } else if &line != "" {
                panic!();
            }
        }

        // println!(
        //     "Monkes:\n{}",
        //     serde_json::to_string_pretty(&monkes).unwrap()
        // );

        monkes
    }
}

fn part1(mut monkes: Vec<Monke>) -> usize {
    for _ in 0..20 {
        for monke_index in 0..monkes.len() {
            for _ in 0..monkes[monke_index].items.len() {
                // Monkey 0:
                //   Monkey inspects an item with a worry level of 79.
                //     Worry level is multiplied by 19 to 1501.
                //     Monkey gets bored with item. Worry level is divided by 3 to 500.
                //     Current worry level is not divisible by 23.
                //     Item with worry level 500 is thrown to monkey 3.

                let monke = &mut monkes[monke_index];

                let item = monke.items.pop_front().unwrap();

                let worry = match monke.op {
                    Operation::Add(operand) => (item + operand) / 3,
                    Operation::Mult(operand) => item * operand / 3,
                    Operation::Power(operand) => item.pow(operand as u32) / 3,
                    _ => {
                        panic!();
                    }
                };

                monke.items_inspected += 1;

                let index = if &worry % monke.test == BigInt::from(0) {
                    monke.true_monke
                } else {
                    monke.false_monke
                };

                monkes[index].items.push_back(worry);
            }
        }
    }

    monkes.sort_by_key(|x| x.items_inspected);
    monkes.reverse();

    monkes[0].items_inspected * monkes[1].items_inspected
}

fn part2(mut monkes: Vec<Monke>) -> usize {
    let base = monkes
        .iter()
        .map(|x| x.test)
        .reduce(|acc, x| acc * x)
        .unwrap();
    for _ in 0..10000 {
        for monke_index in 0..monkes.len() {
            for _ in 0..monkes[monke_index].items.len() {
                // Monkey 0:
                //   Monkey inspects an item with a worry level of 79.
                //     Worry level is multiplied by 19 to 1501.
                //     Monkey gets bored with item. Worry level is divided by 3 to 500.
                //     Current worry level is not divisible by 23.
                //     Item with worry level 500 is thrown to monkey 3.

                let monke = &mut monkes[monke_index];

                let item = monke.items.pop_front().unwrap();

                let worry = match monke.op {
                    Operation::Add(operand) => item + operand,
                    Operation::Mult(operand) => item * operand,
                    Operation::Power(operand) => item.pow(operand as u32),
                    _ => {
                        panic!();
                    }
                } % base;

                monke.items_inspected += 1;

                let index = if &worry % monke.test == BigInt::from(0) {
                    monke.true_monke
                } else {
                    monke.false_monke
                };

                monkes[index].items.push_back(worry);
            }
        }
    }

    monkes.sort_by_key(|x| x.items_inspected);
    monkes.reverse();

    monkes[0].items_inspected * monkes[1].items_inspected
}

fn main() {
    {
        // Asserts
        assert_eq!(part1(Monke::from_file("test")), 10605);
        assert_eq!(part2(Monke::from_file("test")), 2713310158);
    }
    {
        // Part 1
        let ans = part1(Monke::from_file("input"));
        println!("Part 1: Ans is: {}", ans);
    }
    {
        // Part 2
        let ans = part2(Monke::from_file("input"));
        println!("Part 2: Ans is: {}", ans);
    }
}
