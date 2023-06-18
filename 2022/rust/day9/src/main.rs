// #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Debug)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use scanf::sscanf;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
enum Direction {
    EAST,
    NORTH,
    WEST,
    SOUTH,
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            'R' => Direction::EAST,
            'U' => Direction::NORTH,
            'L' => Direction::WEST,
            'D' => Direction::SOUTH,
            _ => todo!(),
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from(x: isize, y: isize) -> Point {
        Point { x: x, y: y }
    }
    fn from_dir(d: &Direction) -> Point {
        match d {
            Direction::EAST => Point { x: 1, y: 0 },
            Direction::NORTH => Point { x: 0, y: 1 },
            Direction::WEST => Point { x: -1, y: 0 },
            Direction::SOUTH => Point { x: 0, y: -1 },
        }
    }

    fn touching(self: &Self, other: &Self) -> bool {
        isize::abs(self.x - other.x) <= 1 && isize::abs(self.y - other.y) <= 1
    }

    fn add(self: &mut Self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }

    fn unit_diff(self: &Self, other: &Self) -> Point {
        Point {
            x: isize::clamp(self.x - other.x, -1, 1),
            y: isize::clamp(self.y - other.y, -1, 1),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Default, Debug)]
struct Instance {
    knots: Vec<Point>,
    visited: std::collections::HashSet<Point>,
    start: Point,
}

impl Instance {
    fn print(self: &Self) {
        let max_dims = self
            .visited
            .iter()
            .cloned()
            .fold(Point::from(0, 0), |a, e| {
                Point::from(isize::max(a.x, e.x), isize::max(a.y, e.y))
            });
        let min_dims = self
            .visited
            .iter()
            .cloned()
            .fold(Point::from(0, 0), |a, e| {
                Point::from(isize::min(a.x, e.x), isize::min(a.y, e.y))
            });
        let max_dims_knots = self.knots.iter().cloned().fold(Point::from(0, 0), |a, e| {
            Point::from(isize::max(a.x, e.x), isize::max(a.y, e.y))
        });
        let min_dims_knots = self.knots.iter().cloned().fold(Point::from(0, 0), |a, e| {
            Point::from(isize::min(a.x, e.x), isize::min(a.y, e.y))
        });

        let max_dims = Point::from(
            isize::max(max_dims.x, max_dims_knots.x),
            isize::max(max_dims.y, max_dims_knots.y),
        );
        let min_dims = Point::from(
            isize::min(min_dims.x, min_dims_knots.x),
            isize::min(min_dims.y, min_dims_knots.y),
        );

        let width = (max_dims.x - min_dims.x) as usize;
        println!("{:>width$}({}, {})", "", max_dims.x + 1, max_dims.y + 1);
        let width = ((max_dims.x - min_dims.x) + 3) as usize;
        println!("{:>width$}", "v");
        for y in (min_dims.y - 1..=max_dims.y + 1).rev() {
            for x in min_dims.x - 1..=max_dims.x + 1 {
                if self.knots[0] == Point::from(x, y) {
                    print!("H");
                } else if self.knots.contains(&Point::from(x, y)) {
                    let c = self
                        .knots
                        .iter()
                        .cloned()
                        .position(|p| p == Point::from(x, y))
                        .unwrap()
                        .to_string();
                    print!("{}", c);
                } else if self.start == Point::from(x, y) {
                    print!("s");
                } else if self.visited.contains(&Point::from(x, y)) {
                    print!("#");
                } else {
                    print!("·");
                }
            }
            println!("");
        }

        println!("^\n{:?}", (min_dims.x - 1, min_dims.y - 1));
        // println!("{:?}", self.visited);
    }

    fn move_head_one_step(self: &mut Self, direction: &Direction) {
        self.knots[0].add(&Point::from_dir(&direction));
        // println!("Head at {:?}", self.head);
        // println!("Tail at {:?}", self.tail);
        // println!("Touching: {}", self.head.touching(&self.tail));

        for index in 0..self.knots.len() - 1 {
            let leader = self.knots[index].clone();
            let tailer = self.knots[index + 1].clone();
            if !leader.touching(&tailer) {
                // println!("Adding to tail {:?}", self.head.unit_diff(&self.tail));
                self.knots[index + 1].add(&leader.unit_diff(&tailer));
                if index == self.knots.len() - 2 {
                    // println!("Tail visiting {:?}", self.tail);
                    self.visited.insert(self.knots[index + 1].clone());
                }
            }
        }

        // self.print();
        // println!("");
    }

    fn move_head(self: &mut Self, direction: Direction, distance: usize) {
        // self.print();
        // println!("");

        for _ in 0..distance {
            self.move_head_one_step(&direction);
        }
    }

    fn read_file(self: &mut Self, path: &str) {
        // println!("Tail visiting {:?}", self.tail);
        self.visited
            .insert(self.knots[self.knots.len() - 1].clone());
        for line in BufReader::new(fs::File::open(path).unwrap())
            .lines()
            .enumerate()
        {
            let line = (line.0, line.1.unwrap());
            let mut direction: char = ' ';
            let mut distance: usize = 0;
            assert!(sscanf!(&line.1, "{} {}", direction, distance).is_ok());
            // println!(" === {:?} ===", line);
            self.move_head(Direction::from(direction), distance);
        }
    }

    fn get_num_pos_visited_by_tail(self: &Self) -> usize {
        self.visited.len()
    }
}

fn main() {
    {
        // Asserts
        let mut instance = Instance::default();
        instance.knots.resize(2, Point::default());
        instance.read_file("test");
        // instance.print();
        assert_eq!(instance.get_num_pos_visited_by_tail(), 13);

        let mut instance = Instance::default();
        instance.knots.resize(10, Point::default());
        instance.read_file("test");
        // instance.print();
        assert_eq!(instance.get_num_pos_visited_by_tail(), 1);

        let mut instance = Instance::default();
        instance.knots.resize(10, Point::default());
        instance.read_file("test2");
        // instance.print();
        assert_eq!(instance.get_num_pos_visited_by_tail(), 36);
    }
    {
        // Part 1
        let mut instance = Instance::default();
        instance.knots.resize(2, Point::default());
        instance.read_file("input");
        // instance.print();
        let ans = instance.get_num_pos_visited_by_tail();
        println!("Part 1: Ans is: {}", ans);
    }
    {
        // Part 2
        let mut instance = Instance::default();
        instance.knots.resize(10, Point::default());
        instance.read_file("input");
        // instance.print();
        let ans = instance.get_num_pos_visited_by_tail();
        println!("Part 2: Ans is: {}", ans);
    }
}
