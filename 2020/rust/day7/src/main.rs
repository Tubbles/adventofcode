use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// recursive stuff
fn contains_my_bag(
    my_bag: &str,
    target: &str,
    map: &BTreeMap<String, Vec<(usize, String)>>,
) -> bool {
    let mut out = false;
    // println!("Looking for {}", target);
    for (_, t) in map.get(target).unwrap().iter() {
        if t == my_bag || contains_my_bag(my_bag, t, map) {
            out = true;
            break;
        }
    }
    out
}

fn count_sub_bags(
    target: &str,
    map: &BTreeMap<String, Vec<(usize, String)>>,
    indent: usize,
) -> usize {
    let mut out = 0;
    // let indent_str = (0..indent * 2).map(|_| " ").collect::<String>();
    // println!("{}Counting bags inside {} bags", indent_str, target);
    for (num, t) in map.get(target).unwrap().iter() {
        // println!("{}Found {} {} bags", indent_str, num, t);
        out += num + num * count_sub_bags(t, map, indent + 1);
    }
    // println!("{}= New total: {}", indent_str, out);
    out
}

fn main() {
    let re_lhs = Regex::new(r"^([a-z]+ [a-z]+)").unwrap();
    let re_rhs = Regex::new(r"([0-9]) ([a-z]+ [a-z]+)").unwrap();
    {
        // Asserts
        let s = "dull silver bags contain 2 striped magenta bags, 2 dark coral bags, 1 bright orange bag, 4 plaid blue bags.";
        let goals = vec![
            "dull silver",
            "2",
            "striped magenta",
            "2",
            "dark coral",
            "1",
            "bright orange",
            "4",
            "plaid blue",
        ];
        let caps = re_lhs.captures(s).unwrap();
        assert_eq!(&caps[1], goals[0]);
        for (i, caps) in re_rhs.captures_iter(s).enumerate() {
            assert_eq!(&caps[1], goals[(2 * i) + 1]);
            assert_eq!(&caps[2], goals[(2 * i) + 2]);
        }
    }

    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut map = BTreeMap::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let caps = re_lhs.captures(&line).unwrap();

        let mut v = Vec::new();
        for caps in re_rhs.captures_iter(&line) {
            v.push((caps[1].parse::<usize>().unwrap(), caps[2].to_string()));
        }

        map.insert(caps[1].to_string(), v);
    }

    {
        // Part 1
        let mut count = 0;
        for (k, _) in &map {
            if contains_my_bag("shiny gold", &k, &map) {
                // println!(" -> My bag found!");
                count += 1;
            }
        }

        println!("Part 1: {}", count);
        // println!("Map is: \n{:?}", map);
    }
    println!("Part 2: {}", count_sub_bags("shiny gold", &map, 0));
}
