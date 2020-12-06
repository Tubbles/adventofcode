use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    {
        // Part 1
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut count = 0;
        let mut set = BTreeSet::new();
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            // println!("{}", line);
            if line.len() == 0 {
                // println!("Finished set. adding {:?} (len = {})", set, set.len());
                count += set.len();
                // println!("New count: {}", count);
                set.clear();
            } else {
                line.chars().for_each(|c| {
                    set.insert(c);
                })
            }
        }
        println!("Part 1: {}", count);
    }
    {
        // Part 2
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut count = 0;
        let mut set = BTreeSet::new();
        let mut people = 0;
        for line in f.lines() {
            let mut newset = BTreeSet::new();
            let line = line.expect("Unable to read line");
            // println!("{}", line);
            if line.len() == 0 {
                // println!("Finished set. adding {:?} (len = {})", set, set.len());
                count += set.len();
                // println!("New count: {}\n===", count);
                set.clear();
                people = 0;
            } else {
                line.chars().for_each(|c| {
                    newset.insert(c);
                });
                if people == 0 {
                    set = newset;
                // println!("Initial: {:?}", set);
                } else {
                    set = newset.intersection(&set).cloned().collect();
                    // println!("post intersection: {:?}", set);
                }
                people += 1;
            }
        }
        println!("Part 2: {}", count);
    }
}
