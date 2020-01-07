use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::iter::Iterator;
use std::cmp;

fn get_distance_to_com(
    orb: &String,
    map: &HashMap<String, String>,
) -> usize {
    let mut counter : usize = 1;
    if *orb == "COM".to_string() { return counter }
    let mut orb = map.get(orb).unwrap();
    loop
    {
        counter += 1;
        if *orb == "COM".to_string() { return counter }
        orb = map.get(orb).unwrap();
    }
}

fn find_santa(
    map: &HashMap<String, String>,
) -> usize {
    // First find out santa's path to COM
    let mut out = 0;
    let mut santa_path : Vec<String> = Vec::new();
    let mut orb = map.get("SAN").unwrap();
    loop {
        santa_path.push(orb.to_string());
        if *orb == "COM".to_string() { break; }
        orb = map.get(orb).unwrap();
    }

    // Then find the first orbital from the path of
    // YOU to COM that coincides with santa's
    let mut orb = map.get("YOU").unwrap();
    loop {
        if santa_path.contains(orb) {
            out += santa_path.iter().position(|r| r == orb).unwrap();
            break;
        }
        orb = map.get(orb).unwrap();
        out += 1; // Use the out var to store our iteration loop
    }
    out
}

fn main() {
    let mut map = HashMap::new();
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut counter = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut splits : Vec<&str> = line.split(")").collect();
        let sat = splits.pop().unwrap().to_string();
        let orb = splits.pop().unwrap().to_string();
        map.insert(sat, orb);
        counter += 1;
    }
    println!("Read {} lines", counter);

    let mut tot: usize = 0;
    let mut max: usize = 0;
    for (_sat, dat) in &map {
        let dist = get_distance_to_com(&dat, &map);
        max = cmp::max(dist, max);
        tot += dist;
    }

    println!("Tree depth = {}", max);
    println!("Total number of orbitals (part 1) = {}", tot);
    println!("You orbit {}", map.get("YOU").unwrap());
    println!("Santa orbits {}", map.get("SAN").unwrap());
    println!("Distance to Santa (part 2) = {}", find_santa(&map));
}
