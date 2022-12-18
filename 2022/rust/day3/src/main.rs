use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn char_to_points(c: &char) -> isize {
    if c.is_lowercase() {
        *c as isize - 'a' as isize + 1
    } else {
        *c as isize - 'A' as isize + 27
    }
}

fn intersect_char(s: &str) -> char {
    let (str1, str2) = (&s[0..s.len() / 2], &s[s.len() / 2..s.len()]);
    let (set1, set2): (BTreeSet<char>, BTreeSet<char>) =
        (str1.chars().collect(), str2.chars().collect());
    set1.intersection(&set2).nth(0).unwrap().to_owned()
}

fn main() {
    {
        // Asserts
        assert!(char_to_points(&'a') == 1);
        assert!(char_to_points(&'z') == 26);
        assert!(char_to_points(&'A') == 27);
        assert!(char_to_points(&'Z') == 52);
    }
    {
        // Part 1
        let mut points = 0;
        for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
            let line = line.unwrap();
            points += char_to_points(&intersect_char(&line));
        }
        println!("Part 1: Ans is: {}", points);
    }
    {
        // Part 2
        for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
            let _line = line.unwrap();
        }
        println!("Part 2: Ans is: {}", 0);
    }
}
