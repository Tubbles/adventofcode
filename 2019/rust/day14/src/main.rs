use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_recipes_from_file(
    recipes: &mut HashMap<String, (isize, Vec<(isize, String)>)>,
    file: &str,
) {
    let re = Regex::new(r"(\d+) ([A-Z]+),?").unwrap();
    //                   7 A, 1 B => 1 C
    let f = File::open(file).expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut caps = re.captures_iter(&line);
        let mut temps = Vec::new();
        loop {
            match caps.next() {
                Some(c) => {
                    temps.push((c[1].parse().unwrap(), c[2].to_string()));
                }
                None => break,
            }
        }
        let (output_amount, output_name) = temps.pop().unwrap();
        // println!("{:?} => {:?}", temps, output);

        recipes.insert(output_name, (output_amount, temps));
    }
}

fn create_chemical(
    recipes: &HashMap<String, (isize, Vec<(isize, String)>)>,
    inventory: &mut HashMap<String, isize>,
    // _bill: &mut HashMap<String, isize>,
    name: &String,
    amount: isize,
) {
    // First modify our inventory
    let entry = inventory.entry(name.to_string()).or_insert(0);
    *entry -= amount;

    // Check if there was a deficit
    if *entry < 0 {
        // Check for recipe
        if recipes.contains_key(name) {
            let (output_amount, ingredients) = &recipes[name];
            // println!("{}({}): {:?}", name, output_amount, ingredients);

            // Check the amount - we need at least as much as the deficit
            let mut times = -*entry / output_amount;
            if times * output_amount < -*entry {
                times += 1;
            }
            // println!("{} times", times);

            // Add the amount and create new deficits
            *entry += times * output_amount;
            for (in_amount, in_name) in ingredients {
                // println!("ingredient: {:?}", (in_amount, in_name));
                create_chemical(
                    recipes,
                    inventory,
                    // _bill,
                    in_name,
                    in_amount * times,
                );
            }
        }
    }
}

fn main() {
    let files = [
        "example1.txt",
        "example2.txt",
        "example3.txt",
        "example4.txt",
        "example5.txt",
        "input.txt",
    ];
    for file in &files {
        println!("{}", file);
        let mut recipes: HashMap<String, (isize, Vec<(isize, String)>)> = HashMap::new(); // output name -> list of input names
        read_recipes_from_file(&mut recipes, file);
        println!("recipes: {:?}", recipes);

        let mut inventory: HashMap<String, isize> = HashMap::new(); // name -> current inventory

        // let mut bill: HashMap<String, isize> = HashMap::new(); // name -> num to create
        // create_chemical(&recipes, &mut inventory, &mut bill, &"FUEL".to_string(), 1); // Create one fuel
        create_chemical(&recipes, &mut inventory, &"FUEL".to_string(), 1); // Create one fuel

        println!("inventory: {:?}", inventory);
    }
}
