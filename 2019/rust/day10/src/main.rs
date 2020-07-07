use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub const PRINT_DEBUG: bool = true;

struct Point {
    x: usize,
    y: usize,
}

// Returns the minimum resolution required for integer approximation of floats for correct
// angle calculations
// Calculate by taking the angle delta between the diagonal and the diagonal + 1 in
// either direction, and take the half of that angle due to shannon.
// The actual resolution is expressed in "number of such circle segments per revolution",
// rounded up to nearest integer
// fn minimum_resolution(width: usize, height: usize) -> usize {
//     // We use law of cotangents to calculate the angle A. Example for a 4x4 diagram:
//     //          C
//     //         *
//     //       * * B
//     //      **
//     //    **
//     // A *
//     // where sides a, b, c are opposite of the point/angle at A, B, C, respectively.
//     // length AB = short_diagonal,
//     // length AC = long_diagonal,
//     // length BC = 1,
//     // a = 2*atan(r/(s-BC)), where s = semi_perimeter, and r = inradius
//     // we return 2*pi/a/2 + 0.5 to round up as integer

//     let (width, height) = (width as f64, height as f64);
//     let (new_width, new_height) = if width > height {
//         (width + 1_f64, height)
//     } else {
//         (width, height + 1_f64)
//     };

//     let short_diagonal = (width.powi(2) + height.powi(2)).sqrt(); // pythagorean theorem
//     let long_diagonal = (new_width.powi(2) + new_height.powi(2)).sqrt();
//     let semi_perimeter = (short_diagonal + long_diagonal + 1_f64) / 2_f64; // + 1 for the last
//     let inradius = ((semi_perimeter - short_diagonal)
//         * (semi_perimeter - long_diagonal)
//         * (semi_perimeter - 1_f64)
//         / semi_perimeter)
//         .sqrt();

//     let angle_a = 2_f64 * (inradius / (semi_perimeter - 1_f64)).atan();
//     (4_f64 * std::f64::consts::PI / angle_a - 0.5_f64) as usize
//     // (10_f64 * 2_f64 * std::f64::consts::PI / angle_a + 0.5_f64) as usize
// }

// Applies the above resolution to convert the f64 to a usable, ordinally correct integer
// fn angle_to_int(angle: f64, resolution: usize) -> usize {
//     let out = (angle / (2_f64 * std::f64::consts::PI) * resolution as f64) as usize;
//     if PRINT_DEBUG {
//         println!("({})", out);
//     }
//     out
// }

// fn get_angle(p1: &Point, p2: &Point, resolution: usize) -> usize {
//     let angle = (p2.y as f64 - p1.y as f64).atan2(p2.x as f64 - p1.x as f64);
//     let angle = if angle < 0_f64 {
//         angle + 2.0_f64 * std::f64::consts::PI
//     } else {
//         angle
//     };

//     if PRINT_DEBUG {
//         let degs = (angle / 2.0_f64 / std::f64::consts::PI * 360.0_f64) as isize;
//         print!(
//             "angle ({},{}) -> ({},{}): {} deg ",
//             p1.x, p1.y, p2.x, p2.y, degs
//         );
//     }

//     angle_to_int(angle, resolution)
// }

// fn get_angle(p1: &Point, p2: &Point) -> Fraction {
//     let angle = (p2.y as f64 - p1.y as f64).atan2(p2.x as f64 - p1.x as f64);
//     let angle = if angle < 0_f64 {
//         angle + 2.0_f64 * std::f64::consts::PI
//     } else {
//         angle
//     };

//     let angle = Fraction::from(angle);

//     if PRINT_DEBUG {
//         let degs = angle / Fraction::from(2.0_f64) / Fraction::from(std::f64::consts::PI)
//             * Fraction::from(360.0_f64);
//         print!(
//             "angle ({},{}) -> ({},{}): {} deg ",
//             p1.x, p1.y, p2.x, p2.y, degs
//         );
//     }
//     angle
// }

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

    pub fn _destroy_comet(&mut self, p: Point) {
        self.v[p.y * self.w + p.x] = '.';
    }

    pub fn print_highlighted_comet(&mut self, p: Point) {
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
        println!()
    }

    pub fn _get_first_comet_at_angle_from_point(&self, _p: Point, _angle: usize) -> Option<Point> {
        None
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
    // let resolution = 10000;
    // let resolution = minimum_resolution(map.w, map.h);

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
                            "angle ({},{}) -> ({},{}): {:?} deg",
                            p1.x, p1.y, p2.x, p2.y, ang
                        );
                    }
                    uniq_angs.insert(ang);
                    angs.push((mdist(&p1, &p2), ang, (x2, y2))); // push all comets
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
        "max: ({},{}) @ {} comets",
        max_p.x,
        max_p.y,
        final_uniq_angs.len()
    );

    final_angs.sort_unstable_by_key(|(d, _, _)| *d);

    println!("all other comets: {:?}", final_angs);

    map.print_highlighted_comet(max_p);

    // Part 2
    // let mut angle = angle_to_int(std::f64::consts::PI / 2_f64, resolution); // pi/2 = straight north

    // let mut x = 0;
    // let mut y = 0;
    // while map.count_comets() > 0 {
    //     map.destroy_comet(Point { x: x, y: y });

    //     // println!("nr comets left: {}", map.count_comets());

    //     x += 1;
    //     if x == map.w {
    //         x = 0;
    //         y += 1;
    //     }

    //     if y == map.h {
    //         println!("end of map reached");
    //     } else {
    //     }

    //     // angle = if angle == 0 { resolution } else { angle - 1 };
    // }

    // max_comets
}
