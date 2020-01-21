use std::fs::File;
use std::io::{BufRead, BufReader};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const PIXELS_PER_LAYER: usize = WIDTH*HEIGHT;

#[derive(Clone, PartialEq)]
struct Layer
{
    pixels : Vec<usize>,
}

fn count_digit(l : &Layer, d : usize) -> usize
{
    let mut count = 0;
    for p in l.pixels.clone()
    {
        if p == d {count += 1;}
    }
    count
}

fn main() {
    let mut layers : Vec<Layer> = Vec::new();
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    // Scan the lines of the file
    let mut digits: Vec<usize> = Vec::new();
    for line in f.lines()
    {
        let line = line.expect("Unable to read line");
        let mut v: Vec<_> = line
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect();
        digits.append(&mut v);
    }

    println!("Pixels per layer: {:?}", PIXELS_PER_LAYER);
    println!("Pixels tot: {:?}", digits.len());
    println!("Number of layers: {:?}", digits.len()/PIXELS_PER_LAYER);

    for chunk in digits.chunks_mut(PIXELS_PER_LAYER)
    {
        layers.push(Layer{pixels:chunk.to_vec()});
    }
    let mut min_layer : Layer = Layer{pixels:Vec::new()};
    let mut layer_count : usize = std::usize::MAX;
    for layer in layers
    {
        let count = count_digit(&layer, 0);
        if count < layer_count
        {
            layer_count = count;
            min_layer = layer;
        }
    }
    println!("Part 1: {}", count_digit(&min_layer, 1) * count_digit(&min_layer, 2));
    assert_eq!(count_digit(&min_layer, 1) * count_digit(&min_layer, 2), 2904);

    
}
