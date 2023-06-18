use scanf::sscanf;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    {
        // Asserts
    }
    {
        // Part 1
        let mut parsing_procedures = false;
        let mut num_stacks = 0;
        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        for line in BufReader::new(File::open("input").unwrap()).lines() {
            let line = line.unwrap();
            if num_stacks == 0 {
                num_stacks = (line.len() + 1) / 4;
                for _ in 0..num_stacks {
                    stacks.push(VecDeque::new());
                }
            }
            if !parsing_procedures && line.contains("1") {
                parsing_procedures = true;
                continue;
            }

            if line.trim() == "" {
                continue;
            }

            if !parsing_procedures {
                for (i, c) in line.chars().enumerate() {
                    let check = (i as isize - 1) % 4;
                    if check == 0 && c != ' ' {
                        let index = i / 4;
                        stacks[index].push_front(c)
                    }
                }
                // Parsing stacks
            } else {
                // Parsing procedures
                let mut num = 0;
                let mut from = 0;
                let mut to = 0;
                assert!(sscanf!(&line, "move {} from {} to {}", num, from, to).is_ok());
                let from = from - 1;
                let to = to - 1;

                // Execute
                for _ in 0..num {
                    let c = stacks[from].pop_back().unwrap();
                    stacks[to].push_back(c);
                    // println!("moved {} from {} to {}", c, from, to);
                }
            }
        }

        let mut ans = String::new();
        for i in 0..stacks.len() {
            ans.push(stacks[i][stacks[i].len() - 1]);
            // println!("{:?}", stacks[i]);
        }

        println!("Part 1: Ans is: {}", ans);
    }
    {
        // Part 2
        let mut parsing_procedures = false;
        let mut num_stacks = 0;
        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        for line in BufReader::new(File::open("input").unwrap()).lines() {
            let line = line.unwrap();
            if num_stacks == 0 {
                num_stacks = (line.len() + 1) / 4;
                for _ in 0..num_stacks {
                    stacks.push(VecDeque::new());
                }
            }
            if !parsing_procedures && line.contains("1") {
                parsing_procedures = true;
                // println!("     1    2    3    4    5    6    7    8    9   10");
                // for i in 0..stacks.len() {
                //     println!("{}: {:?}", i + 1, stacks[i]);
                // }
                continue;
            }

            if line.trim() == "" {
                continue;
            }

            if !parsing_procedures {
                for (i, c) in line.chars().enumerate() {
                    let check = (i as isize - 1) % 4;
                    if check == 0 && c != ' ' {
                        let index = i / 4;
                        stacks[index].push_front(c)
                    }
                }
                // Parsing stacks
            } else {
                // Parsing procedures
                let mut num = 0;
                let mut from = 0;
                let mut to = 0;
                assert!(sscanf!(&line, "move {} from {} to {}", num, from, to).is_ok());
                let from = from - 1;
                let to = to - 1;

                // Execute
                let at = stacks[from].len() - num;
                let mut v = stacks[from].split_off(at);
                stacks[to].append(&mut v);

                // println!("\n{}\n", line);
                // println!("     1    2    3    4    5    6    7    8    9   10");
                // for i in 0..stacks.len() {
                //     println!("{}: {:?}", i + 1, stacks[i]);
                // }
            }
        }

        let mut ans = String::new();
        for i in 0..stacks.len() {
            ans.push(stacks[i][stacks[i].len() - 1]);
            // println!("{:?}", stacks[i]);
        }

        println!("Part 2: Ans is: {}", ans);
    }
}
