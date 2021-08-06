// use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_differences(v: Vec<usize>) -> Vec<usize> {
    let mut out = Vec::new();
    for i in 0..v.len() {
        if i == 0 {
            continue; // cannot calculate difference from the last
        }
        let index = v[i] - v[i - 1];
        // println!(
        //     "diff: i = {:?}, v[i] = {:?}, v[i - 1] = {:?}, diff = {:?}",
        //     i,
        //     v[i],
        //     v[i - 1],
        //     index
        // );
        if index > out.len() {
            out.resize(index + 1, 0);
        }
        out[v[i] - v[i - 1]] += 1;
    }
    out
}

// fn permutate(v: Vec<usize>) -> usize {
//     let mut out = 0;
//     // println!("Permutating: \t{:?}", v);
//     if v.len() == 1 {
//         out += 1;
//     }
//     if v.len() > 1 && v[1] - v[0] <= 3 {
//         // println!("Recurse1: \t{:?}", v[1..].to_vec());
//         out += permutate(v[1..].to_vec());
//     }
//     if v.len() > 2 && v[2] - v[0] <= 3 {
//         // println!("Recurse2: \t{:?}", v[2..].to_vec());
//         out += permutate(v[2..].to_vec());
//     }
//     if v.len() > 3 && v[3] - v[0] <= 3 {
//         // println!("Recurse3: \t{:?}", v[3..].to_vec());
//         out += permutate(v[3..].to_vec());
//     }
//     // println!("Permutating: \t{:?}, out: {:?}", v, out);
//     out
// }

fn permutate2(v: Vec<usize>) -> usize {
    let mut out = 1;
    let mut i = 0;
    loop {
        println!("out: {:?}, i: {:?}", out, i);
        let mut perms = 0;
        let mut skips = 0;

        if i < v.len() - 3 {
            match (
                v[i + 1] == v[i] + 1,
                v[i + 2] == v[i] + 2,
                v[i + 3] == v[i] + 3,
            ) {
                (true, true, true) => {
                    perms = 4;
                    skips = 4;
                }
                (true, true, false) => {
                    perms = 2;
                    skips = 3;
                }
                (true, false, true) => {
                    perms = 2;
                    skips = 3;
                }
                (true, false, false) => {
                    perms = 1;
                    skips = 2;
                }
                (false, true, true) => {
                    perms = 2;
                    skips = 3;
                }
                (false, true, false) => {
                    perms = 1;
                    skips = 2;
                }
                (false, false, true) => {
                    perms = 1;
                    skips = 2;
                }
                (false, false, false) => {
                    println!("Should never happen3");
                }
            }
        } else if i < v.len() - 2 {
            match (v[i + 1] == v[i] + 1, v[i + 2] == v[i] + 2) {
                (true, true) => {
                    perms = 2;
                    skips = 3;
                }
                (true, false) => {
                    perms = 1;
                    skips = 2;
                }
                (false, true) => {
                    perms = 1;
                    skips = 2;
                }
                (false, false) => {
                    println!("Should never happen2");
                }
            }
        } else if i < v.len() - 1 {
            match v[i + 1] == v[i] + 1 {
                true => {
                    perms = 1;
                    skips = 2;
                }
                false => {
                    println!("Should never happen1");
                }
            }
        }
        // println!("v[i + 3] ({:?}) - v[i + 0] ({:?})", v[i + 3], perms);
        println!("skips: {:?}, perms: {:?}", skips, perms);

        out *= perms;
        i += skips;
        if i >= v.len() {
            break;
        }
    }
    out
}

fn main() {
    {
        // Tests
        let f = File::open("test1.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut vec = Vec::new();
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            let entry = line.parse::<usize>().expect("Unable to parse line");
            vec.push(entry);
        }
        vec.push(0);
        vec.sort();
        vec.push(vec.last().unwrap() + 3);
        let diff = get_differences(vec);
        println!("test 1: {:?}", diff);
        assert!(diff[1] == 7);
        assert!(diff[3] == 5);
    }
    {
        // Tests
        let f = File::open("test2.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut vec = Vec::new();
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            let entry = line.parse::<usize>().expect("Unable to parse line");
            vec.push(entry);
        }
        vec.push(0);
        vec.sort();
        vec.push(vec.last().unwrap() + 3);
        let diff = get_differences(vec);
        // println!("test 2: {:?}", diff);
        assert!(diff[1] == 22);
        assert!(diff[3] == 10);
    }
    {
        // Part 1
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut vec = Vec::new();
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            let entry = line.parse::<usize>().expect("Unable to parse line");
            vec.push(entry);
        }
        vec.push(0);
        vec.sort();
        vec.push(vec.last().unwrap() + 3);
        let diff = get_differences(vec);
        println!("Part 1: diffs: {:?}", diff);
        println!("Part 1: Ans is: {}", diff[1] * diff[3]);
    }
    {
        // Tests
        let f = File::open("test1.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut vec = Vec::new();
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            let entry = line.parse::<usize>().expect("Unable to parse line");
            vec.push(entry);
        }
        vec.push(0);
        vec.sort();
        vec.push(vec.last().unwrap() + 3);
        let perm = permutate2(vec);
        println!("test 1: {:?}", perm);
        assert!(perm == 8);
    }
    {
        // Part 2
        // let f = File::open("input.txt").expect("Unable to open file");
        // let f = BufReader::new(f);
        // let mut vec = Vec::new();
        // for line in f.lines() {
        //     let line = line.expect("Unable to read line");
        //     let entry = line.parse::<usize>().expect("Unable to parse line");
        //     vec.push(entry);
        // }
        // vec.push(0);
        // vec.sort();
        // vec.push(vec.last().unwrap() + 3);
        // let perm = permutate(vec);
        // println!("Part 2: Ans is: {}", perm);
    }
}
