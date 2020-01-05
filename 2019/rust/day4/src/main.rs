use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input_range() -> (usize, usize)
{
    let mut lower : usize = 100000;
    let mut upper : usize = 999999;
    // Grab the limits
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    // Scan the first line of the file
    for line in f.lines()
    {
        let line = line.expect("Unable to read line");
        let splits : Vec<&str> = line.split("-").collect();
        lower = splits[0].parse::<usize>().expect("Unable to parse split");
        upper = splits[1].parse::<usize>().expect("Unable to parse split");
    }
    (lower, upper)
}

// The criterias to meet are:
// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
fn meets_criteria(a: usize, lower: usize, upper: usize) -> bool
{
    let mut res = true;
    // let (lower, upper) = get_input_range();
    let digits : Vec<_> = a.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    // Check bounds
    // We assume here that the lower bound is an 6 digit number, so
    // it automatically accounts also for that criteria
    if a > upper || a < lower { res = false }

    // Check if at least two digits next to each other are the same
    let mut identical_adjacents = false;
    for i in 0..digits.len()-1
    {
        if digits[i] == digits[i+1]
        {
            identical_adjacents = true;
        }
        if digits[i+1] < digits[i] { res = false }
    }
    if !identical_adjacents { res = false }

    res
}

// Additionally, for part 2 the doubly digits cannot be part
// of a triplet or larger
fn meets_criteria_part2(a: usize, lower: usize, upper: usize) -> bool
{
    let mut res = true;
    // let (lower, upper) = get_input_range();
    let digits : Vec<_> = a.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    // Check bounds
    // We assume here that the lower bound is an 6 digit number, so
    // it automatically accounts also for that criteria
    if a > upper || a < lower { res = false }

    // Check if at least two digits next to each other are the same
    let mut identical_adjacents = false;
    for i in 0..digits.len()-1
    {
        if i == 0
        {
            if  (digits[i] == digits[i+1]) &&
                (digits[i] != digits[i+2])
            {
                // println!("{} adjacent at {}", a, i);
                identical_adjacents = true;
            }
        }
        else if i < digits.len()-2
        {
            if  (digits[i] == digits[i+1]) &&
                (digits[i] != digits[i+2]) &&
                (digits[i] != digits[i-1])
            {
                // println!("{} adjacent at {}", a, i);
                identical_adjacents = true;
            }
        }
        else
        {
            if  (digits[i] == digits[i+1]) &&
                (digits[i] != digits[i-1])
            {
                // println!("{} adjacent at {}", a, i);
                identical_adjacents = true;
            }
        }
    }
    if !identical_adjacents { res = false }

    for i in 0..digits.len()-1
    {
        if digits[i+1] < digits[i] { res = false }
    }

    res
}

fn main()
{
    let (lower, upper) = get_input_range();

    // We use debug assert here so that we can do cargo run --release to run without the assertions
    // debug_assert!(!meets_criteria(111111));
    debug_assert!(!meets_criteria(223450, lower, upper));
    debug_assert!(!meets_criteria(123789, lower, upper));
    debug_assert!(!meets_criteria(123, lower, upper));
    debug_assert!(meets_criteria(134566, lower, upper));

    debug_assert!(meets_criteria_part2(223344, lower, upper));
    debug_assert!(!meets_criteria_part2(234555, lower, upper));
    debug_assert!(meets_criteria_part2(222233, lower, upper));
    debug_assert!(!meets_criteria_part2(222234, lower, upper));
    debug_assert!(!meets_criteria_part2(234567, lower, upper));
    debug_assert!(!meets_criteria_part2(234555, lower, upper));
    debug_assert!(!meets_criteria_part2(222222, lower, upper));
    debug_assert!(meets_criteria_part2(234566, lower, upper));
    println!("Asserts passed");

    {
        let mut num_pass = 0;
        for number in lower..upper
        {
            if meets_criteria(number, lower, upper)
            {
                num_pass = num_pass+1;
            }
        }
        println!("Num passes part1: {}", num_pass);
    }
    {
        let mut num_pass = 0;
        for number in lower..upper
        {
            if meets_criteria_part2(number, lower, upper)
            {
                num_pass = num_pass+1;
            }
        }
        // Answer is not 1837
        println!("Num passes part2: {}", num_pass);
    }
}
