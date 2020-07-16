use std::fs::File;
use std::io::prelude::*;

fn str_to_intvec(s: &str) -> Vec<isize> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}

fn get_first_8(v: &Vec<isize>) -> Vec<isize> {
    v.into_iter().take(8).map(|&a| a).collect()
}

fn phase(v: &Vec<isize>, iters: usize) -> Vec<isize> {
    const BASE: [isize; 4] = [0, 1, 0, -1];
    let mut acc = 0;
    let mut outvec = Vec::new();
    let mut invec = v.clone();

    for _ in 0..iters {
        outvec = invec.clone();
        for out_i in 0..invec.len() {
            for in_i in 0..invec.len() {
                acc += invec[in_i]
                    * BASE[((in_i as f64 + 1f64) / (out_i as f64 + 1f64)) as usize % BASE.len()];
            }
            outvec[out_i] = acc.abs() % 10;
            acc = 0;
        }
        invec = outvec.clone();
    }
    outvec
}

fn main() {
    let small_examples_in = ["12345678", "48226158", "34040438", "03415518"];

    let small_examples_out = ["48226158", "34040438", "03415518", "01029498"];

    let long_examples_in = [
        "80871224585914546619083218645595",
        "19617804207202209144916044189917",
        "69317163492948606335995924319873",
    ];

    let long_examples_out = ["24176176", "73745418", "52432133"];

    for i in 0..small_examples_in.len() {
        println!();
        let res = phase(&str_to_intvec(small_examples_in[i]), 1);
        let res = get_first_8(&res);
        let assert = str_to_intvec(small_examples_out[i]);
        println!("small example num {}: {:?} == {:?}", i + 1, res, assert);
        assert!(res == assert);
    }

    for i in 0..long_examples_in.len() {
        println!();
        let res = phase(&str_to_intvec(long_examples_in[i]), 100);
        let res = get_first_8(&res);
        let assert = str_to_intvec(long_examples_out[i]);
        println!("long example num {}: {:?} == {:?}", i + 1, res, assert);
        assert!(res == assert);
    }

    let mut file = File::open("input.txt").expect("open file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read file error");
    let res = phase(&str_to_intvec(&contents), 100);
    let res = get_first_8(&res);

    println!();
    println!("part1: {:?}", res);
}
