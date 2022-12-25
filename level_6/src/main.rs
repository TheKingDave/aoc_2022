use std::fs::File;
use std::io::{Read};

fn main() {
    let mut file = File::open("../inputs/input_6.txt").expect("File not found");
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let chars = input.chars().collect::<Vec<char>>();

    // calc_start(chars, 4);
    calc_start(chars, 14);
}

fn calc_start(chars: Vec<char>, window_size: usize) {
    let count = chars.windows(window_size).take_while(|c| {
        for i in 0..window_size {
            for n in i+1..window_size {
                if c[i] == c[n] {
                    return true;
                }
            }
        }
        false
    }).count();

    println!("Count: {}", count + window_size);
}