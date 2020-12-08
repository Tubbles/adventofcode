use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let re = Regex::new(r"^(acc|jmp|nop) ([+-][0-9]+)").unwrap();
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut v = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        // println!("{}", line);
        let caps = re.captures(&line).unwrap();
        v.push((caps[1].to_string(), caps[2].parse::<isize>().unwrap(), 0));
    }
    let v = v;
    let len = v.len();

    {
        // Part 1
        let mut acc = 0;
        let mut pc: isize = 0;
        let mut v_copy = v.clone();

        loop {
            // println!("acc: {}, pc: {}\n{:?}", acc, pc, v);
            let (op, arg, num) = &mut v_copy[pc as usize];
            if *num > 0 {
                break;
            }
            *num += 1;
            match op.as_str() {
                "acc" => {
                    acc += *arg;
                    pc += 1;
                }
                "jmp" => {
                    pc += *arg;
                }
                _ => {
                    pc += 1;
                }
            }
        }
        // println!("acc: {}, pc: {}\n{:?}", acc, pc, v);
        println!("Part 1: {}", acc);
    }
    {
        // Part 2
        let mut acc = 0;
        for (i, _) in v.iter().enumerate() {
            if v[i].0.as_str() == "acc" {
                continue;
            }
            let mut v_copy = v.clone();

            if v_copy[i].0.as_str() == "nop" {
                v_copy[i].0 = "jmp".to_string();
            } else {
                v_copy[i].0 = "nop".to_string();
            }

            acc = 0;
            let mut pc: isize = 0;

            // println!("start i: {}", i);
            loop {
                if pc >= len as isize {
                    // println!("pc break @ acc: {}, pc: {}\n{:?}", acc, pc, v_copy);
                    break;
                }
                // let inner_v = v_copy.clone();
                let (op, arg, num) = &mut v_copy[pc as usize];
                // println!(
                //     "acc: {}, pc: {} ({:?})\n{:?}",
                //     &acc,
                //     &pc,
                //     (&op, &arg, &num),
                //     &inner_v
                // );
                if *num > 0 {
                    // println!(
                    //     "num break @ acc: {}, pc: {} ({:?})\n{:?}",
                    //     &acc,
                    //     &pc,
                    //     (&op, &arg, &num),
                    //     &inner_v
                    // );
                    break;
                }
                *num += 1;
                match op.as_str() {
                    "acc" => {
                        acc += *arg;
                        pc += 1;
                    }
                    "jmp" => {
                        pc += *arg;
                    }
                    _ => {
                        pc += 1;
                    }
                }
            }

            if pc >= v.len() as isize {
                break;
            }
        }
        println!("Part 2: {}", acc);
    }
}
