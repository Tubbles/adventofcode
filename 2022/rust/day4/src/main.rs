use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn new(s: isize, e: isize) -> Range {
        Range { start: s, end: e }
    }
}

fn to_ranges(s: &str) -> (Range, Range) {
    let s1: Vec<&str> = s.split(",").collect();
    let l: Vec<isize> = s1[0]
        .split("-")
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    let r: Vec<isize> = s1[1]
        .split("-")
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    (Range::new(l[0], l[1]), Range::new(r[0], r[1]))
}

fn fully_overlaps(r1: &Range, r2: &Range) -> bool {
    r1.start <= r2.start && r1.end >= r2.end
}

fn overlaps(r1: &Range, r2: &Range) -> bool {
    (r1.start <= r2.start && r1.end >= r2.start) || (r1.start <= r2.end && r1.end >= r2.end)
}

fn main() {
    {
        // Asserts
        assert!(to_ranges("2-4,3-12") == (Range::new(2, 4), Range::new(3, 12)));
        assert!(fully_overlaps(&Range::new(2, 4), &Range::new(3, 3)) == true);
        assert!(fully_overlaps(&Range::new(2, 4), &Range::new(3, 4)) == true);
        assert!(fully_overlaps(&Range::new(2, 4), &Range::new(2, 3)) == true);
        assert!(fully_overlaps(&Range::new(2, 4), &Range::new(3, 5)) == false);
        assert!(fully_overlaps(&Range::new(2, 4), &Range::new(1, 4)) == false);
        assert!(fully_overlaps(&Range::new(2, 4), &Range::new(6, 12)) == false);
        assert!(overlaps(&Range::new(2, 4), &Range::new(3, 3)) == true);
        assert!(overlaps(&Range::new(2, 4), &Range::new(3, 4)) == true);
        assert!(overlaps(&Range::new(2, 4), &Range::new(2, 3)) == true);
        assert!(overlaps(&Range::new(2, 4), &Range::new(3, 5)) == true);
        assert!(overlaps(&Range::new(2, 4), &Range::new(1, 4)) == true);
        assert!(overlaps(&Range::new(2, 4), &Range::new(6, 12)) == false);
    }
    {
        // Part 1
        let mut num_overlaps = 0;
        for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
            let line = line.unwrap();
            let ranges = to_ranges(&line);
            num_overlaps +=
                if fully_overlaps(&ranges.0, &ranges.1) || fully_overlaps(&ranges.1, &ranges.0) {
                    1
                } else {
                    0
                };
        }
        println!("Part 1: Ans is: {}", num_overlaps);
    }
    {
        // Part 2
        let mut num_overlaps = 0;
        for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
            let line = line.unwrap();
            let ranges = to_ranges(&line);
            num_overlaps += if overlaps(&ranges.0, &ranges.1) || overlaps(&ranges.1, &ranges.0) {
                1
            } else {
                0
            };
        }
        println!("Part 2: Ans is: {}", num_overlaps);
    }
}
