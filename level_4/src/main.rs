use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../inputs/input_4.txt").expect("File not found");
    let buf_reader = BufReader::new(file);

    let count = buf_reader
        .lines()
        .map(|line| parse_pair(line.unwrap()))
        .filter(|pair| is_partially_contained(pair.to_owned()))
        .count();

    println!("{}", count);
}

#[allow(dead_code)]
fn is_fully_contained(pair: ((i32, i32), (i32, i32))) -> bool {
    pair.0.0 >= pair.1.0 && pair.0.0 <= pair.1.1 || pair.0.1 <= pair.0.0 && pair.0.1 >= pair.0.1
}

fn is_partially_contained(pair: ((i32, i32), (i32, i32))) -> bool {
    pair.0.0 >= pair.1.0 && pair.0.0 <= pair.1.1 || pair.0.1 >= pair.1.0 && pair.0.1 <= pair.1.1 ||
        pair.1.0 >= pair.0.0 && pair.1.0 <= pair.0.1 || pair.1.1 >= pair.0.0 && pair.1.1 <= pair.0.1
}

fn parse_pair(str: String) -> ((i32, i32), (i32, i32)) {
    let pair = str.split_once(",").unwrap();
    (parse_range(pair.0), parse_range(pair.1))
}

fn parse_range(str: &str) -> (i32, i32) {
    let range = str.split_once("-").unwrap();
    (range.0.parse::<i32>().unwrap(), range.1.parse::<i32>().unwrap())
}