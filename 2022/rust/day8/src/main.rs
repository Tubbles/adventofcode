// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]

use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Debug)]
struct TreeMap {
    trees: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl TreeMap {
    fn from_file(path: &str) -> TreeMap {
        let lines: Vec<String> = BufReader::new(fs::File::open(path).unwrap())
            .lines()
            .map(|x| x.unwrap())
            .collect();

        let lines_ref = lines.iter().map(|x| x.as_str()).collect();

        TreeMap::from(lines_ref)
    }

    fn from(lines: Vec<&str>) -> TreeMap {
        let mut map = TreeMap::default();

        for (line_num, line) in lines.iter().enumerate() {
            map.trees.push(Vec::new());
            for c in line.chars() {
                let digit = c.to_digit(10).unwrap() as usize;
                map.trees[line_num].push(digit);
            }
        }

        map.width = map.trees[0].len();
        map.height = map.trees.len();

        map
    }

    // fn print(self: &Self) {
    //     println!("width = {}, height = {}", self.width, self.height);
    //     for row in &self.trees {
    //         for tree in row {
    //             print!("{}", tree);
    //         }
    //         println!("");
    //     }
    // }

    fn at(self: &Self, x: usize, y: usize) -> usize {
        self.trees[y][x]
    }

    fn tree_is_visible(self: &Self, x: usize, y: usize) -> bool {
        let height = self.at(x, y);
        // println!("Is tree {} at {:?} visible? ", height, (x, y));

        if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
            // println!(" - Yes");
            return true;
        }

        // Check east
        for test in x + 1..self.width {
            // println!(" - Checking E {:?} ", (test, y));

            if self.at(test, y) >= height {
                // println!(" - No ({} >= {})", self.at(test, y), height);
                break;
            }

            if test == self.width - 1 {
                // println!(" - Yes");
                return true;
            }
        }

        // Check north
        for test in (0..y).rev() {
            // println!(" - Checking N {:?} ", (x, test));

            if self.at(x, test) >= height {
                // println!(" - No ({} >= {})", self.at(x, test), height);
                break;
            }

            if test == 0 {
                // println!(" - Yes");
                return true;
            }
        }

        // Check west
        for test in (0..x).rev() {
            // println!(" - Checking W {:?} ", (test, y));

            if self.at(test, y) >= height {
                // println!(" - No ({} >= {})", self.at(test, y), height);
                break;
            }

            if test == 0 {
                // println!(" - Yes");
                return true;
            }
        }

        // Check south
        for test in y + 1..self.height {
            // println!(" - Checking S {:?} ", (x, test));

            if self.at(x, test) >= height {
                // println!(" - No ({} >= {})", self.at(x, test), height);
                break;
            }

            if test == self.height - 1 {
                // println!(" - Yes");
                return true;
            }
        }

        // println!(" - Not visible");
        false
    }

    fn count_visible(self: &Self) -> usize {
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.tree_is_visible(x, y) {
                    count += 1;
                }
            }
        }

        count
    }

    fn scenic_score_at(self: &Self, x: usize, y: usize) -> usize {
        let height = self.at(x, y);
        let mut score = 1;
        let mut section_score;
        // println!("Scenic score of {} at {:?}? ", height, (x, y));

        // Check east
        section_score = 0;
        for test in x + 1..self.width {
            // println!(" - Checking E {:?} ", (test, y));
            section_score += 1;
            // println!(" - Direction score is {} ", section_score);

            if self.at(test, y) >= height {
                // println!(" - Final ({} >= {})", self.at(test, y), height);
                break;
            }
        }
        score *= section_score;

        // Check north
        section_score = 0;
        for test in (0..y).rev() {
            // println!(" - Checking N {:?} ", (x, test));
            section_score += 1;
            // println!(" - Direction score is {} ", section_score);

            if self.at(x, test) >= height {
                // println!(" - Final ({} >= {})", self.at(x, test), height);
                break;
            }
        }
        score *= section_score;

        // Check west
        section_score = 0;
        for test in (0..x).rev() {
            // println!(" - Checking W {:?} ", (test, y));
            section_score += 1;
            // println!(" - Direction score is {} ", section_score);

            if self.at(test, y) >= height {
                // println!(" - Final ({} >= {})", self.at(test, y), height);
                break;
            }
        }
        score *= section_score;

        // Check south
        section_score = 0;
        for test in y + 1..self.height {
            // println!(" - Checking S {:?} ", (x, test));
            section_score += 1;
            // println!(" - Direction score is {} ", section_score);

            if self.at(x, test) >= height {
                // println!(" - Final ({} >= {})", self.at(x, test), height);
                break;
            }
        }
        score *= section_score;

        // println!(" - Score is {}", score);
        score
    }

    fn highest_scenic_score(self: &Self) -> usize {
        let mut highest = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                highest = usize::max(highest, self.scenic_score_at(x, y));
            }
        }

        highest
    }
}

fn main() {
    {
        // Asserts
        {
            let map = TreeMap::from_file("test");
            // map.print();
            assert_eq!(map.count_visible(), 21);
            assert_eq!(map.highest_scenic_score(), 8);
        }
        {
            #[rustfmt::skip]
            let map = TreeMap::from(vec![
                "123",
                "123",
                "123"
            ]);
            // map.print();
            assert_eq!(map.count_visible(), 9);
            assert_eq!(map.highest_scenic_score(), 1);
        }
        {
            #[rustfmt::skip]
            let map = TreeMap::from(vec![
                "333",
                "323",
                "333"
            ]);
            // map.print();
            assert_eq!(map.count_visible(), 8);
            assert_eq!(map.highest_scenic_score(), 1);
        }
        {
            #[rustfmt::skip]
            let map = TreeMap::from(vec![
                "1233",
                "1233",
                "1233"
            ]);
            // map.print();
            assert_eq!(map.count_visible(), 12);
            assert_eq!(map.highest_scenic_score(), 2);
        }
        {
            #[rustfmt::skip]
            let map = TreeMap::from(vec![
                "1433",
                "1433",
                "1533"
            ]);
            // map.print();
            assert_eq!(map.count_visible(), 11);
            assert_eq!(map.highest_scenic_score(), 2);
        }
    }
    let map = TreeMap::from_file("input");
    // map.print();
    {
        // Part 1
        let ans = map.count_visible();
        println!("Part 1: Ans is: {}", ans);
    }
    {
        // Part 2
        let ans = map.highest_scenic_score();
        println!("Part 2: Ans is: {}", ans);
    }
}
