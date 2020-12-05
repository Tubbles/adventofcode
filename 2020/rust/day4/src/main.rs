use std::fs::File;
use std::io::{BufRead, BufReader};

// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)

// enum Keys {
//     Byr = 0,
//     Iyr,
//     Eyr,
//     Hgt,
//     Hcl,
//     Ecl,
//     Pid,
//     Cid,
// }

const KEY_NAMES: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn main() {
    {
        // Part 1
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut num_valids = 0;
        let mut keys_found: [bool; 8] = [false; 8];
        // let mut line_counter = 0;
        let mut num_passports = 0;
        for line in f.lines() {
            // line_counter += 1;
            // println!("line_counter: {}", line_counter);
            // let mut input_splits = Vec::new();
            let line = line.expect("Unable to read line");

            if line.len() == 0 {
                // New record
                let mut current_is_valid = true;
                for i in 0..7 {
                    if keys_found[i] == false {
                        current_is_valid = false; // Check fields
                                                  // println!("Invalid, missing: {}", KEY_NAMES[i]);
                        break;
                    } else {
                        // println!("Valid: {}", KEY_NAMES[i]);
                    }
                }
                if current_is_valid == true {
                    num_valids += 1; // Count number valids
                }
                keys_found = [false; 8];
                // println!("");
                num_passports += 1;
            }

            for i in 0..8 {
                if line.contains(&(KEY_NAMES[i].to_owned() + ":")[..]) {
                    keys_found[i] = true;
                }
            }
        }
        println!("Part 1: Num valids: {}", num_valids);
        println!("Part 1: Num passports: {}", num_passports);
    }
    {
        // Part 2
        let f = File::open("input.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        let mut num_valids = 0;
        let mut keys_found: [String; 8] = Default::default();
        let mut line_counter = 0;
        let mut num_passports = 0;
        for line in f.lines() {
            line_counter += 1;
            println!("line_counter: {}", line_counter);
            // let mut input_splits = Vec::new();
            let line = line.expect("Unable to read line");

            if line.len() == 0 {
                // New record
                let mut current_is_valid = true;
                for i in 0..7 {
                    if keys_found[i] == "".to_string() {
                        current_is_valid = false; // Check fields
                        println!("Invalid, missing: {}", KEY_NAMES[i]);
                        break;
                    } else {
                        println!(
                            "Present: {}:{}, checking validity",
                            KEY_NAMES[i], keys_found[i]
                        );
                        current_is_valid = match i {
                            0 => {
                                // byr
                                (1920..2020 + 1)
                                    .contains(&keys_found[i].parse().expect("Unable to parse line"))
                            }
                            1 => {
                                // iyr
                                (2010..2020 + 1)
                                    .contains(&keys_found[i].parse().expect("Unable to parse line"))
                            }
                            2 => {
                                // eyr
                                (2020..2030 + 1)
                                    .contains(&keys_found[i].parse().expect("Unable to parse line"))
                            }
                            3 => {
                                // hgt
                                if keys_found[i].ends_with("cm") {
                                    keys_found[i].pop();
                                    keys_found[i].pop();
                                    (150..193 + 1).contains(
                                        &keys_found[i].parse().expect("Unable to parse line"),
                                    )
                                } else if keys_found[i].ends_with("in") {
                                    keys_found[i].pop();
                                    keys_found[i].pop();
                                    (59..76 + 1).contains(
                                        &keys_found[i].parse().expect("Unable to parse line"),
                                    )
                                } else {
                                    false
                                }
                            }
                            4 => {
                                // hcl
                                if keys_found[i].starts_with("#") {
                                    keys_found[i].remove(0);
                                    match usize::from_str_radix(&keys_found[i], 16) {
                                        Ok(_) => true,
                                        _ => false,
                                    }
                                } else {
                                    false
                                }
                            }
                            5 => {
                                // ecl
                                if keys_found[i] == "amb"
                                    || keys_found[i] == "blu"
                                    || keys_found[i] == "brn"
                                    || keys_found[i] == "gry"
                                    || keys_found[i] == "grn"
                                    || keys_found[i] == "hzl"
                                    || keys_found[i] == "oth"
                                {
                                    true
                                } else {
                                    false
                                }
                            }
                            6 => {
                                // pid
                                if keys_found[i].len() != 9 {
                                    false
                                } else {
                                    keys_found[i].parse::<usize>().is_ok()
                                }
                            }
                            _ => false,
                        };
                        println!("{} is {:?}", KEY_NAMES[i], current_is_valid);
                        if current_is_valid == false {
                            break;
                        }
                    }
                }
                if current_is_valid == true {
                    num_valids += 1; // Count number valids
                }
                keys_found = Default::default();
                println!("");
                num_passports += 1;
            }

            let entries: Vec<_> = line.split(" ").collect();
            for e in entries {
                let splits: Vec<_> = e.split(":").collect();
                for i in 0..8 {
                    if splits[0].trim() == KEY_NAMES[i] {
                        keys_found[i] = splits[1].trim().to_string();
                    }
                }
            }
        }
        println!("Part 2: Num valids: {}", num_valids);
        println!("Part 2: Num passports: {}", num_passports);
    }
}
