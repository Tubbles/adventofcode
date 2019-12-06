use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use std::convert::TryInto;
use std::iter::FromIterator;

type Coord = (isize, isize);
type Sect = (Coord, Coord);
type Dirs = (char, isize);

fn intersection(s1: Sect, s2: Sect) -> Option<Coord>
{
    /*
    minx1 < (minx2 == maxx2) < maxx1 &&
    miny2 < (miny1 == maxy1) < maxy2
    - OR -
    minx2 < (minx1 == maxx1) < maxx2 &&
    miny1 < (miny2 == maxy2) < maxy1
    */

    let mut ret : Option<Coord> = None;

    let minx1 = cmp::min((s1.0).0, (s1.1).0);
    let maxx1 = cmp::max((s1.0).0, (s1.1).0);
    let miny1 = cmp::min((s1.0).1, (s1.1).1);
    let maxy1 = cmp::max((s1.0).1, (s1.1).1);
    let minx2 = cmp::min((s2.0).0, (s2.1).0);
    let maxx2 = cmp::max((s2.0).0, (s2.1).0);
    let miny2 = cmp::min((s2.0).1, (s2.1).1);
    let maxy2 = cmp::max((s2.0).1, (s2.1).1);

    // First case
    if
    (minx2 == maxx2) &&
    (minx1 < minx2) &&
    (maxx2 < maxx1) &&
    (miny1 == maxy1) &&
    (miny2 < miny1) &&
    (maxy1 < maxy2)
    {
        ret = Some((minx2, miny1));
    }
    // First case
    else if
    (minx1 == maxx1) &&
    (minx2 < minx1) &&
    (maxx1 < maxx2) &&
    (miny2 == maxy2) &&
    (miny1 < miny2) &&
    (maxy2 < maxy1)
    {
        ret = Some((minx1, miny2));
    }
    ret
}

fn manhattan_distance(p1: Coord, p2: Coord) -> usize
{
    let deltax = p2.0-p1.0;
    let deltay = p2.1-p1.1;
    let absx = deltax.abs();
    let absy = deltay.abs();
    let tot = absx + absy;
    tot.try_into().unwrap()
}

fn track_length(v: &[Coord]) -> usize
{
    let mut len = 0;
    if v.len() > 0 {for i in 0..v.len()-1
    {
        len += manhattan_distance(v[i], v[i+1]);
    }}
    len
}

// From https://stackoverflow.com/questions/30811107/getting-a-single-character-out-of-a-string#comment83958722_48482196
// Thanks https://stackoverflow.com/users/155423/shepmaster !
fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn calc(filestr: &str, prints: bool) -> (usize, usize)
{
    // Part 1
    // let f = File::open("ex1.txt").expect("Unable to open file");
    let f = File::open(filestr).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut wires : Vec<Vec<Coord>> = Vec::new();
    let mut directs : Vec<Vec<Dirs>> = Vec::new();

    // Scan the lines of the file and create coordinates directly
    for line in f.lines()
    {
        let mut wire : Vec<Coord> = Vec::new();
        let mut direction : Vec<Dirs> = Vec::new();
        let mut last_coord : Coord = (0,0); // Start at 0
        wire.push(last_coord); // Push origo
        let line = line.expect("Unable to read line");
        let splits : Vec<&str> = line.split(",").collect();
        for split in splits
        {
            let (dir, len) = car_cdr(split);
            let dir = dir.chars().nth(0).unwrap();
            let len = len.parse::<isize>().expect("Unable to parse segment");
            direction.push((dir, len));
            match dir
            {
                'R' => { last_coord = (last_coord.0+len,last_coord.1) },
                'L' => { last_coord = (last_coord.0-len,last_coord.1) },
                'U' => { last_coord = (last_coord.0,last_coord.1+len) },
                'D' => { last_coord = (last_coord.0,last_coord.1-len) },
                c   => { println!("Faulty character: {}", c) },
            }
            wire.push(last_coord);
        }
        wires.push(wire);
        directs.push(direction);
    }

    let mut num_checks : usize = 0;
    let mut num_isects : usize = 0;
    let mut closest_distance : usize = std::usize::MAX;
    let mut closest_distance_by_track_length : usize = std::usize::MAX;

    if prints {for dir in directs
    {
        println!("{:?}, {}", dir, dir.len());
    }}

    if prints {for wire in &wires
    {
        println!("{:?}, {}", wire, wire.len());
    }}

    // Check for intersections
    for w1 in 0..wires.len()
    {
        for w2 in w1+1..wires.len()
        {
            for c1 in 0..wires[w1].len()-1
            {
                for c2 in 0..wires[w2].len()-1
                {
                    if prints {println!("Comparing ({:?}->{:?}) with ({:?}->{:?})", wires[w1][c1], wires[w1][c1+1], wires[w2][c2], wires[w2][c2+1])}
                    num_checks += 1;
                    match intersection((wires[w1][c1], wires[w1][c1+1]), (wires[w2][c2], wires[w2][c2+1]))
                    {
                        None => {},
                        Some(c) => {
                            num_isects += 1;
                            let cd = manhattan_distance((0,0), c);
                            closest_distance = cmp::min(closest_distance, cd);

                            let mut track1 = Vec::from_iter(wires[w1][0..c1+1].iter().cloned());
                            let mut track2 = Vec::from_iter(wires[w2][0..c2+1].iter().cloned());
                            track1.push(c);
                            track2.push(c);
                            let tl = track_length(&track1)+track_length(&track2);
                            closest_distance_by_track_length = cmp::min(closest_distance_by_track_length, tl);

                            if prints {println!("\tIntersection found at {:?}: {:?}", c, (cd, tl))}
                            if prints {println!("\tOrig1:  {:?}, {}", wires[0], wires[0].len());}
                            if prints {println!("\tOrig2:  {:?}, {}", wires[1], wires[1].len());}
                            if prints {println!("\tTrack1: {:?}, {}", track1, track1.len())}
                            if prints {println!("\tTrack2: {:?}, {}", track2, track2.len())}
                        },
                    }
                }
            }
        }
    }

    println!("Read {} complete ({} checks, {} isects), got: {:?}", filestr, num_checks, num_isects, (closest_distance, closest_distance_by_track_length));
    (closest_distance, closest_distance_by_track_length)
}

fn main()
{
    // Unit tests
    assert!(intersection(((1,0),(1,2)), ((3,3),(4,4))) == None);
    assert!(intersection(((1,0),(1,2)), ((0,1),(2,1))) == Some((1,1)));
    assert!(intersection(((1,0),(1,2)), ((0,1),(-2,1))) == None);

    assert!(manhattan_distance((0,0), (1,1)) == 2);
    assert!(manhattan_distance((0,0), (2,2)) == 4);
    assert!(manhattan_distance((-2,-2), (2,2)) == 8);
    assert!(manhattan_distance((2,-2), (2,2)) == 4);
    assert!(manhattan_distance((-2,2), (2,2)) == 4);
    assert!(manhattan_distance((3,3), (3,3)) == 0);
    
    assert!(track_length(&[(2,0),(1,0),(1,2)]) == 3);
    assert!(track_length(&[(0,0)]) == 0);
    assert!(track_length(&[]) == 0);
    assert!(track_length(&[(0,0),(-10,0),(-13,0),(-13,20),(0,20),(0,0)]) == 66);

    assert!(calc("ex1.txt", false) == (159, 610));
    assert!(calc("ex2.txt", false) == (135, 410));
    calc("input.txt", false);
}
