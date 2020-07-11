use float_cmp::approx_eq;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub const PRINT_DEBUG: bool = false;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct Starmap {
    v: Vec<char>,
    w: usize,
    h: usize,
}

impl Starmap {
    pub fn new() -> Self {
        Starmap {
            v: Vec::new(),
            w: 0,
            h: 0,
        }
    }

    pub fn get_pos(&self, p: &Point) -> char {
        self.v[p.y * self.w + p.x]
    }

    pub fn load_from_file(&mut self, file: &str) {
        let f = BufReader::new(File::open(file).expect("Unable to open file"));

        for line in f.lines() {
            let line = line.expect("Unable to read line");
            let mut v: Vec<_> = line.chars().collect();
            self.v.append(&mut v);
            self.w = line.len();
            self.h += 1;
        }
    }

    pub fn count_comets(&self) -> usize {
        let mut out = 0;
        for c in &self.v {
            if *c == '#' {
                out += 1;
            }
        }
        out
    }

    pub fn destroy_comet(&mut self, p: &Point) {
        self.v[p.y * self.w + p.x] = '.';
    }

    pub fn print_highlighted_comet(&mut self, p: &Point) {
        const CL_RED: &str = "\x1B[34m";
        const CL_FG: &str = "\x1B[0m";
        for i in 0..self.w * self.h {
            if i % self.w == 0 {
                println!()
            }
            if i == p.y * self.w + p.x {
                print!("{}{}{}", CL_RED, '#', CL_FG);
            } else {
                print!("{}", self.v[i]);
            }
        }
        println!();
        println!("num comets: {}", self.count_comets());
    }
}

fn gcd((a, b): (isize, isize)) -> isize {
    // Terminal cases
    if a == b {
        return a;
    }
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let a_is_even = a % 2 == 0;
    let b_is_even = b % 2 == 0;

    match (a_is_even, b_is_even) {
        (true, true) => gcd((a / 2, b / 2)) * 2,
        (true, false) => gcd((a / 2, b)),
        (false, true) => gcd((a, b / 2)),
        (false, false) => {
            if a > b {
                gcd(((a - b) / 2, b))
            } else {
                gcd(((b - a) / 2, a))
            }
        }
    }
}

fn reduce((a, b): (isize, isize)) -> (isize, isize) {
    // Use absolute value because negatives
    let gcd = gcd((a.abs(), b.abs()));

    (a / gcd, b / gcd)
}

// manhattan distance
fn mdist(p1: &Point, p2: &Point) -> usize {
    ((p2.x as isize - p1.x as isize).abs() + (p2.y as isize - p1.y as isize).abs()) as usize
}

fn main() {
    let mut map: Starmap = Starmap::new();
    map.load_from_file("input.txt");

    let mut max_p: Point = Point { x: 0, y: 0 };
    let mut max_comets: usize = 0;
    let mut final_uniq_angs = BTreeSet::new();
    let mut final_angs = Vec::new();

    for x1 in 0..map.w {
        for y1 in 0..map.h {
            let p1 = Point { x: x1, y: y1 };

            if map.get_pos(&p1) != '#' {
                continue;
            }

            let mut uniq_angs = BTreeSet::new();
            let mut angs = Vec::new();

            for x2 in 0..map.w {
                for y2 in 0..map.h {
                    let p2 = Point { x: x2, y: y2 };

                    if map.get_pos(&p2) != '#' {
                        continue;
                    }

                    if x1 == x2 && y1 == y2 {
                        continue;
                    }

                    let ang =
                        reduce(((x2 as isize) - (x1 as isize), (y2 as isize) - (y1 as isize)));
                    if PRINT_DEBUG {
                        println!(
                            "angle ({}, {}) -> ({}, {}): {:?} deg",
                            p1.x, p1.y, p2.x, p2.y, ang
                        );
                    }
                    uniq_angs.insert(ang);
                    angs.push((mdist(&p1, &p2), ang, (p2.x, p2.y))); // push all comets
                }
            }

            if PRINT_DEBUG {
                println!("uniq_angs: ({}) {:?}\n", uniq_angs.len(), uniq_angs);
            }

            if uniq_angs.len() > max_comets {
                max_p = p1;
                max_comets = uniq_angs.len();
                final_uniq_angs = uniq_angs;
                final_angs = angs;
            }
        }
    }

    println!("w: {}, h: {}", map.w, map.h);
    println!("nr comets: {}", map.count_comets());
    println!(
        "max: ({}, {}) @ {} comets",
        max_p.x,
        max_p.y,
        final_uniq_angs.len()
    );

    final_angs.sort_unstable_by_key(|(d, _, _)| *d);

    println!("all other comets: {:?}", final_angs);

    print!("Scanner station deployed at:");
    map.print_highlighted_comet(&max_p);

    // Part 2
    let mut interactive = true;
    let num_starting_comets = map.count_comets();

    let mut all_angles = Vec::new();
    for (dist, (angx, angy), (posx, posy)) in final_angs {
        let angle = (angx as f64).atan2(-angy as f64);
        let angle = if angle < 0f64 {
            angle + 2.0f64 * std::f64::consts::PI
        } else {
            angle
        };
        let angle = angle * 360f64 / 2.0f64 / std::f64::consts::PI;
        let ang = (angle, (angx, angy), dist, (posx, posy), false); // last bool == is_destroyed
        all_angles.push(ang);
    }

    all_angles.sort_by_key(|(_, _, a, _, _)| *a); // sort by dist
    all_angles.sort_by(|(a, _, _, _, _), (b, _, _, _, _)| a.partial_cmp(b).unwrap()); // sort by angle

    for ang in &all_angles {
        println!("{:?}", ang);
    }

    let mut last_realang = 360f64;
    let mut i = 0;
    while map.count_comets() > 1 {
        if i == all_angles.len() {
            i = 0;
        }
        let (realang, (_, _), _, (x, y), destroyed) = &mut all_angles[i];
        let comet = Point { x: *x, y: *y };
        if approx_eq!(f64, last_realang, *realang, ulps = 2) {
            i += 1;
            continue;
        }
        if *destroyed == true {
            i += 1;
            continue;
        }

        map.print_highlighted_comet(&comet);
        if interactive {
            println!(
                "Press enter to vaporize comet #{} @ {:?}",
                num_starting_comets - map.count_comets() + 1,
                comet
            );
            print!("or send RUN to exit interactive mode > ");
            std::io::stdout().flush().ok();
            let mut line = String::new();
            let _input = std::io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            if line == "RUN\n" {
                interactive = false;
            }
        }

        last_realang = *realang;
        map.destroy_comet(&comet);
        *destroyed = true;
        i += 1;
    }
}
