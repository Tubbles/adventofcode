use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

static DEBUG: bool = false;

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
        if DEBUG {
            println!("{:?} => {} {}", temps, output_amount, output_name);
        }

        recipes.insert(output_name, (output_amount, temps));
    }
}

fn put_indents(indents: usize) {
    for _ in 0..indents {
        print!(" ");
    }
}

fn create_bill(
    recipes: &HashMap<String, (isize, Vec<(isize, String)>)>,
    inventory: &mut HashMap<String, isize>,
    name: &String,
    amount: isize,
) -> bool // Return whether we succesfully crafted it
{
    if DEBUG {
        println!("BILL: {:?}", (amount, name));
    }
    return create_chemical(recipes, inventory, name, amount, true, 0);
}

fn create_dependency(
    recipes: &HashMap<String, (isize, Vec<(isize, String)>)>,
    inventory: &mut HashMap<String, isize>,
    name: &String,
    amount: isize,
    indent: usize,
) -> bool // Return whether we succesfully crafted it
{
    let entry = inventory.entry(name.to_string()).or_insert(0);
    // First check the stock
    let non_neg_entry = isize::max(*entry, 0);
    // Then create the deficit
    *entry -= amount;

    let deficit = amount - non_neg_entry;
    if deficit <= 0 {
        // Consume the chemical
        if DEBUG {
            put_indents(indent);
            println!(
                "{} is sufficiently stocked in the inventory ({}/{})",
                name, non_neg_entry, amount
            );
        }
        return true;
    } else {
        if DEBUG {
            put_indents(indent);
            println!(
                "{} is NOT sufficiently stocked in the inventory ({}/{})",
                name, non_neg_entry, amount
            );
        }
        // We try to create the chemical instead
        if !create_chemical(recipes, inventory, name, deficit, false, indent) {
            if DEBUG {
                put_indents(indent);
                let entry = inventory.entry(name.to_string()).or_insert(0);
                println!(
                        "could not synthesize dependency ingredient: {:?} (no recipe found, creating deficit instead (total {}))",
                        (amount, name), *entry
                    );
            }
            return false;
        }
        return true;
    }
}

fn create_chemical(
    recipes: &HashMap<String, (isize, Vec<(isize, String)>)>,
    inventory: &mut HashMap<String, isize>,
    name: &String,
    amount: isize,
    is_bill: bool,
    indent: usize,
) -> bool // Return whether we succesfully crafted it
{
    if DEBUG {
        put_indents(indent);
        println!("creating {} of {}", amount, name);
    }

    // Check for recipe
    if recipes.contains_key(name) {
        let (output_amount, ingredients) = &recipes[name];
        if DEBUG {
            put_indents(indent);
            println!(
                "Recipe found: {}({}) <= {:?}",
                name, output_amount, ingredients
            );
        }

        // Check the amount - we need at least as much as the deficit
        let times = (amount + output_amount - 1) / output_amount;
        if DEBUG {
            put_indents(indent);
            println!("{} times required (total {})", times, times * output_amount);
        }

        for (in_amount, in_name) in ingredients {
            if DEBUG {
                put_indents(indent);
                println!(
                    "checking ingredient: {:?} x {}",
                    (in_amount, in_name),
                    times
                );
            }

            // See if we can create the dependency chemicals
            create_dependency(recipes, inventory, in_name, in_amount * times, indent + 4);
        }
        let stocking = if is_bill {
            times * output_amount
        } else {
            times * output_amount - amount
        };
        {
            let entry = inventory.entry(name.to_string()).or_insert(0);
            *entry += times * output_amount;
            if DEBUG {
                let billed = if is_bill { " (billed)" } else { "" };
                put_indents(indent);
                println!(
                    "successfully synthesized: {}(/{} needed{}, stocking {} to a total of {}) of {}",
                    times * output_amount,
                    amount,
                    billed,
                    stocking,
                    *entry,
                    name
                );
            }
        }
        return true;
    } else {
        // No recipe exists
        return false;
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

    let part1_asserts = [31, 165, 13312, 180697, 2210736, 378929];

    println!("  Part 1");
    for i in 0..files.len() {
        println!("\n {}", files[i]);
        let mut recipes: HashMap<String, (isize, Vec<(isize, String)>)> = HashMap::new(); // output name -> (output amount, list of input amounts and names)
        read_recipes_from_file(&mut recipes, files[i]);

        let mut inventory: HashMap<String, isize> = HashMap::new(); // name -> current inventory

        create_bill(&recipes, &mut inventory, &"FUEL".to_string(), 1); // Create one fuel

        println!("inventory: {:?}", inventory);
        println!(
            "assert!({} == {})",
            part1_asserts[i],
            -inventory.get("ORE").unwrap()
        );
        assert!(part1_asserts[i] == -inventory.get("ORE").unwrap());
    }

    // For part 2 we do some kind of search thing
    println!("  Part 2");
    for i in 0..files.len() {
        println!("\n {}", files[i]);
        let ore_amount = 1_000_000_000_000; // one trillion
        let mut fuel_amount = ore_amount / part1_asserts[i]; // lower bound
        let mut fuel_amount_last = 0;
        let mut delta = 1_000_000; // each time we go over our target we halve the delta and restart from last good value

        let mut recipes: HashMap<String, (isize, Vec<(isize, String)>)> = HashMap::new(); // output name -> (output amount, list of input amounts and names)
        read_recipes_from_file(&mut recipes, files[i]);

        while delta != 0 {
            let mut inventory: HashMap<String, isize> = HashMap::new(); // name -> current inventory
            inventory.insert("ORE".to_string(), ore_amount);

            create_bill(&recipes, &mut inventory, &"FUEL".to_string(), fuel_amount);

            match *inventory.get("ORE").unwrap() {
                d if d > 0 => {
                    fuel_amount_last = fuel_amount;
                    fuel_amount = fuel_amount + delta;
                }
                d if d < 0 => {
                    fuel_amount = fuel_amount_last;
                    delta /= 2;
                }
                _ => {
                    delta = 0;
                }
            };
        }

        // Do a run with the correct values
        let mut inventory: HashMap<String, isize> = HashMap::new(); // name -> current inventory
        inventory.insert("ORE".to_string(), ore_amount);
        create_bill(&recipes, &mut inventory, &"FUEL".to_string(), fuel_amount);
        println!("inventory: {:?}", inventory);
    }
}
