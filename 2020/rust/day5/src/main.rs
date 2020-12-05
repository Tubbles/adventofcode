use std::fs::File;
use std::io::{BufRead, BufReader};

// F = 0
// B = 1
// L = 0
// R = 1

fn get_seat_id(s: &str) -> usize {
    s.chars()
        .fold((0, 2_usize.pow(9)), |(acc, mul), c| {
            // println!("folding: {} @ {:?}", c, (acc, mul));
            (
                acc + (mul
                    * (match c {
                        'F' | 'L' => 0,
                        _ => 1,
                    } as usize)),
                mul >> 1,
            )
        })
        .0
}

fn main() {
    assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
    assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
    assert_eq!(get_seat_id("BBFFBBFRLL"), 820);

    {
        // Part 1
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut v = Vec::new();
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            // println!("{}", line);
            v.push(get_seat_id(&line));
        }
        v.sort();
        println!("Part 1: Highest seat: {}", v.last().unwrap());
        println!(
            "Part 2: My seat: {}",
            v.iter()
                .cloned()
                .fold((0, 0, 0), |(p1, p2, ans), t| {
                    // println!("folding: {} @ {:?}", t, (p1, p2, ans));
                    if p1 + 1 == p2 && p2 + 2 == t {
                        (p2, t, t - 1)
                    } else {
                        (p2, t, ans)
                    }
                })
                .2
        );
    }
}
