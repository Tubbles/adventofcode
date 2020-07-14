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

// fn create_chemical(
//     recipes: &HashMap<String, (isize, Vec<(isize, String)>)>,
//     inventory: &mut HashMap<String, isize>,
//     // _bill: &mut HashMap<String, isize>,
//     name: &String,
//     amount: isize,
//     allow_negative: bool,
// ) -> bool // Returns whether the recipe was afforded in the inventory
// {
//     let entry: isize = match inventory.get(&name.to_string()) {
//         Some(&a) => a,
//         None => 0,
//     };

//     // Check if we can afford it directly
//     if entry >= amount {
//         // We have enough already in the inventory
//         let entry_mut = inventory.entry(name.to_string()).or_insert(0);
//         *entry_mut -= amount;
//         return true;
//     } else {
//         // Check for recipe
//         if recipes.contains_key(name) {
//             let (output_amount, ingredients) = &recipes[name];
//             // println!("{}({}): {:?}", name, output_amount, ingredients);

//             // Get the amount needed to produce
//             let mut times = -entry / output_amount;
//             if times * output_amount < -entry {
//                 times += 1;
//             }
//             // println!("{} times", times);

//             // Check if we can afford the recipe
//             let mut afforded = true;
//             for (in_amount, in_name) in ingredients {
//                 // println!("ingredient: {:?}", (in_amount, in_name));
//                 let temp_afford = create_chemical(
//                     recipes,
//                     inventory,
//                     in_name,
//                     in_amount * times,
//                     allow_negative,
//                 );
//                 if temp_afford == false {
//                     afforded = false;
//                 }
//             }
//             let entry_mut = inventory.entry(name.to_string()).or_insert(0);
//             if afforded {
//                 // Do the actual chemical reaction
//                 *entry_mut += times * output_amount;
//                 // We now have enough in the inventory
//                 *entry_mut -= amount;
//                 return true;
//             } else if allow_negative {
//                 // We create a deficit
//                 *entry_mut -= amount;
//                 return false;
//             } else {
//                 // Could not afford
//                 return false;
//             }
//         } else {
//             // No recipe found
//             return false;
//         }
//     }
// }

// fn create_chemical(
//     recipes: &HashMap<String, (isize, Vec<(isize, String)>)>,
//     inventory: &mut HashMap<String, isize>,
//     name: &String,
//     amount: isize,
// ) {
//     // First modify our inventory
//     let entry = inventory.entry(name.to_string()).or_insert(0);
//     *entry -= amount;

//     // Check if there was a deficit
//     if *entry < 0 {
//         // Check for recipe
//         if recipes.contains_key(name) {
//             let (output_amount, ingredients) = &recipes[name];
//             // println!("{}({}): {:?}", name, output_amount, ingredients);

//             // Check the amount - we need at least as much as the deficit
//             let mut times = -*entry / output_amount;
//             if times * output_amount < -*entry {
//                 times += 1;
//             }
//             // println!("{} times", times);

//             // Add the amount and create new deficits
//             *entry += times * output_amount;
//             for (in_amount, in_name) in ingredients {
//                 // println!("ingredient: {:?}", (in_amount, in_name));
//                 create_chemical(recipes, inventory, in_name, in_amount * times);
//             }
//         }
//     }
// }

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
        // if times * output_amount < amount {
        //     times += 1;
        // }
        if DEBUG {
            put_indents(indent);
            println!("{} times required (total {})", times, times * output_amount);
        }

        // Add the amount and create new deficits
        // {
        //     let entry = inventory.entry(name.to_string()).or_insert(0);
        //     *entry += times * output_amount;
        // }
        for (in_amount, in_name) in ingredients {
            // Remove the ingredient list
            // *inventory.entry(in_name.to_string()).or_insert(0) -= in_amount * times;
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
        // println!("recipes: {:?}", recipes);

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

    // println!("  Part 2");
    // for i in 0..files.len() {
    //     println!("\n {}", files[i]);
    //     let mut recipes: HashMap<String, (isize, Vec<(isize, String)>)> = HashMap::new(); // output name -> (output amount, list of input amounts and names)
    //     read_recipes_from_file(&mut recipes, files[i]);
    //     // println!("recipes: {:?}", recipes);

    //     let mut inventory: HashMap<String, isize> = HashMap::new(); // name -> current inventory
    //     inventory.insert("ORE".to_string(), 1_000_000_000_000);

    //     while *inventory.get("ORE").unwrap() > 0 {
    //         create_bill(&recipes, &mut inventory, &"FUEL".to_string(), 1_000_000);
    //         // if *inventory.get("ORE").unwrap() % 10_000_000 == 0 {
    //         //     println!("Ore left: {}", *inventory.get("ORE").unwrap());
    //         // }
    //     }

    //     println!("inventory: {:?}", inventory);
    // }

    // println!("  Play area");
    // let file = "example1.txt";
    // println!(" {}", file);
    // let mut recipes: HashMap<String, (isize, Vec<(isize, String)>)> = HashMap::new(); // output name -> (output amount, list of input amounts and names)
    // read_recipes_from_file(&mut recipes, file);
    // println!("recipes: {:?}", recipes);

    // let mut inventory: HashMap<String, isize> = HashMap::new(); // name -> current inventory

    // create_bill(&recipes, &mut inventory, &"FUEL".to_string(), 1); // Create one fuel
    // println!("inventory: {:?}", inventory);

    // create_bill(&recipes, &mut inventory, &"FUEL".to_string(), -1); // Withdraw one fuel
    // println!("inventory: {:?}", inventory);

    // create_bill(&recipes, &mut inventory, &"FUEL".to_string(), 1); // Create one fuel
    // println!("inventory: {:?}", inventory);

    // println!("ORE deficit: {}", -inventory.get("ORE").unwrap());
}
