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

fn intersect_char_vec(v: &Vec<String>) -> char {
    let (set1, set2, set3): (BTreeSet<char>, BTreeSet<char>, BTreeSet<char>) = (
        v[0].chars().collect(),
        v[1].chars().collect(),
        v[2].chars().collect(),
    );

    let vec: Vec<BTreeSet<char>> = vec![set1, set2, set3];
    vec.iter()
        .skip(1)
        .fold(vec[0].clone(), |acc: BTreeSet<char>, hs| {
            acc.intersection(hs).cloned().collect()
        })
        .iter()
        .nth(0)
        .unwrap()
        .to_owned()
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
        let mut count = 0;
        let mut vec = Vec::new();
        let mut points = 0;
        for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
            let line = line.unwrap();
            vec.push(line);
            count += 1;
            if count == 3 {
                points += char_to_points(&intersect_char_vec(&vec));
                vec.clear();
                count = 0;
            }
        }
        println!("Part 2: Ans is: {}", points);
    }
}
