use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    {
        // Part 1
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut num_trees = 0;
        let mut posx = 0;
        // let mut vec = Vec::new();
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            // println!("line: {}", line);

            if line.chars().nth(posx).unwrap() == '#' {
                num_trees += 1;
            }

            posx += 3;

            if posx >= line.chars().count() {
                posx -= line.chars().count();
            }
            // vec.push(line);
        }
        println!("Part 1: Num trees: {}", num_trees);
    }
    {
        // Part 2
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut num_trees: [usize; 5] = [0; 5];
        let mut posx: [usize; 5] = [0; 5];
        let mut right_slope: [usize; 5] = [1, 3, 5, 7, 0];
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            // println!("line: {}", line);

            for i in 0..5 {
                if i != 4 {
                    if line.chars().nth(posx[i]).unwrap() == '#' {
                        num_trees[i] += 1;
                    }
                } else {
                    if right_slope[i] == 0 {
                        if line.chars().nth(posx[i]).unwrap() == '#' {
                            num_trees[i] += 1;
                        }
                        right_slope[i] = 1;
                    } else {
                        right_slope[i] = 0;
                    }
                }

                posx[i] += right_slope[i];

                if posx[i] >= line.chars().count() {
                    posx[i] -= line.chars().count();
                }
            }
        }
        println!(
            "Part 2: Num trees: {}",
            num_trees[0] * num_trees[1] * num_trees[2] * num_trees[3] * num_trees[4]
        );
    }
}
