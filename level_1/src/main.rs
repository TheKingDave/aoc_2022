use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../inputs/input_1.txt").expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut calories: Vec<i32> = vec![];

    let mut current_sum = 0;
    for line_result in buf_reader.lines() {
        let line = line_result.expect("Error while reading lines");
        if line == "" {
            calories.push(current_sum);
            current_sum = 0;
            continue;
        }

        current_sum += line.parse::<i32>().expect("Could not parse to int");
    }

    calories.sort();
    calories.reverse();
    println!("First 3: {:?}", calories.iter().take(3).collect::<Vec<&i32>>());
    println!("Sum {:?}", calories.iter().take(3).sum::<i32>());
}
