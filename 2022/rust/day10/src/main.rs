// #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
// #[rustfmt::skip]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use scanf::sscanf;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
enum MachineState {
    #[default]
    IDLE,
    ADDX(isize),
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
enum Operation {
    #[default]
    NOOP,
    ADDX(isize),
}

impl Operation {
    fn from(s: &str) -> Operation {
        let mut operand_int: isize = 0;
        if s == "noop" {
            return Operation::NOOP;
        } else if sscanf!(s, "addx {}", operand_int).is_ok() {
            return Operation::ADDX(operand_int);
        }

        todo!();
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Debug)]
struct CRT {
    width: usize,
    height: usize,
    pixels: Vec<Vec<char>>,
}

impl CRT {
    fn from(width: usize, height: usize) -> CRT {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        v2.resize(width, ' ');
        v1.resize(height, v2.clone());
        CRT {
            width,
            height,
            pixels: v1,
        }
    }

    fn print(self: &Self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.pixels[y][x]);
            }
            println!("");
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Default, Debug)]
struct Machine {
    crt: CRT,
    state: MachineState,
    cycle: usize,
    operation: Operation,
    register_x: isize,
    signal_strength_sum: isize,
}

impl Machine {
    fn new() -> Machine {
        let mut machine = Machine::default();

        machine.register_x = 1;
        machine.crt = CRT::from(40, 6);

        machine
    }

    fn run_program_file(self: &mut Self, path: &str) {
        let mut ops: Vec<String> = BufReader::new(fs::File::open(path).unwrap())
            .lines()
            .map(|s| s.unwrap())
            .collect();

        ops.reverse();

        while !ops.is_empty() {
            self.cycle += 1;

            let x_pos = ((self.cycle - 1) % self.crt.width) as isize;
            let y_pos = ((self.cycle - 1) / self.crt.width) as isize;
            if x_pos >= self.register_x - 1 && x_pos <= self.register_x + 1 {
                self.crt.pixels[y_pos as usize][x_pos as usize] = '█';
            }

            if (self.cycle as isize - 20) % 40 == 0 {
                self.signal_strength_sum += self.register_x * self.cycle as isize;
            }
            // println!("Cycle {}, X = {}", self.cycle, self.register_x);

            match self.state {
                MachineState::IDLE => {
                    self.operation = Operation::from(ops.pop().unwrap().as_str());

                    match self.operation {
                        Operation::NOOP => {
                            //
                        }

                        Operation::ADDX(strength) => {
                            self.state = MachineState::ADDX(strength);
                        }
                    }
                }

                MachineState::ADDX(strength) => {
                    self.register_x += strength;
                    self.state = MachineState::IDLE;
                }
            }
        }
    }
}

fn main() {
    {
        // Asserts
        let mut machine = Machine::new();
        machine.run_program_file("test");
        assert_eq!(machine.signal_strength_sum, 13140);
        machine.crt.print();
    }
    println!("");
    {
        // Part 1
        let mut machine = Machine::new();
        machine.run_program_file("input");
        machine.crt.print();
        let ans = machine.signal_strength_sum;
        println!("Part 1: Ans is: {}", ans);
    }
    {
        // Part 2
        let ans = 0;
        println!("Part 2: Ans is: {}", ans);
    }
}
