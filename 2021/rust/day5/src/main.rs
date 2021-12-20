use num_traits::pow;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_coords(coords: &str) -> Point {
        let splits = coords.split(",").collect::<Vec<_>>();
        Point {
            x: splits[0].parse().unwrap(),
            y: splits[1].parse().unwrap(),
        }
    }

    fn angle(&self, other: &Point) -> (isize, isize) {
        let x = match other.x as isize - self.x as isize {
            d if d > 0 => 1,
            d if d == 0 => 0,
            d if d < 0 => -1,
            _ => 0,
        };
        let y = match other.y as isize - self.y as isize {
            d if d > 0 => 1,
            d if d == 0 => 0,
            d if d < 0 => -1,
            _ => 0,
        };
        (x, y)
    }

    fn add(&mut self, other: &(isize, isize)) {
        let new_x = self.x as isize + other.0;
        let new_y = self.y as isize + other.1;
        self.x = new_x as usize;
        self.y = new_y as usize;
    }

    fn equals(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    stop: Point,
}

impl Line {
    fn from_segments(segments: &str) -> Line {
        let splits = segments.split(" -> ").collect::<Vec<_>>();
        let l = Line {
            start: Point::from_coords(splits[0]),
            stop: Point::from_coords(splits[1]),
        };
        l
    }

    fn get_points(&self, allow_diagonals: bool) -> Vec<Point> {
        let angle = self.start.angle(&self.stop);
        let mut out = Vec::new();
        if !allow_diagonals && angle.0.abs() + angle.1.abs() > 1 {
            return out;
        }
        let mut p = self.start;

        // println!("start {:?}", p);
        loop {
            // println!("{:?}", p);
            out.push(p.clone());
            if !p.equals(&self.stop) {
                p.add(&angle);
            } else {
                break;
            }
        }
        // println!("stop {:?}", p);

        out
    }
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    points: Vec<usize>,
}

impl Map {
    fn from_dimension(length: usize) -> Map {
        Map {
            width: length,
            points: vec![0; length * length],
        }
    }

    fn apply_line(&mut self, line: &Line, allow_diagonals: bool) {
        for p in line.get_points(allow_diagonals) {
            // println!("{:?}", p);
            self.points[p.x + p.y * self.width] += 1;
        }
    }

    fn num_at_least_two(&self) -> usize {
        self.points
            .iter()
            .filter_map(|&x| if x >= 2 { Some(1) } else { None })
            .sum()
    }
}

fn part1(lines: &Vec<String>) -> usize {
    let dimension = lines[0].split(',').collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>()
        .len();
    let dimension = pow(10, dimension);
    // println!("dimension = {}", dimension);
    let mut map = Map::from_dimension(dimension);
    for line in lines {
        map.apply_line(&Line::from_segments(line.as_str()), false);
    }
    // println!("{:?}", map);
    map.num_at_least_two()
}

fn part2(lines: &Vec<String>) -> usize {
    let dimension = lines[0].split(',').collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>()
        .len();
    let dimension = pow(10, dimension);
    // println!("dimension = {}", dimension);
    let mut map = Map::from_dimension(dimension);
    for line in lines {
        map.apply_line(&Line::from_segments(line.as_str()), true);
    }
    // println!("{:?}", map);
    map.num_at_least_two()
}

fn main() -> Result<(), std::io::Error> {
    let f = BufReader::new(File::open("test1.txt").unwrap());
    let lines = f.lines().filter_map(|s| s.ok()).collect::<Vec<_>>();
    println!("Test Part 1: Ans is: {}", part1(&lines));
    println!("Test Part 2: Ans is: {}", part2(&lines));

    let f = BufReader::new(File::open("input.txt").unwrap());
    let lines = f.lines().filter_map(|s| s.ok()).collect::<Vec<_>>();
    println!("Part 1: Ans is: {}", part1(&lines));
    println!("Part 2: Ans is: {}", part2(&lines));

    Ok(())
}
