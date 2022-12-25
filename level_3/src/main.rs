extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate itertools;

use itertools::Itertools;

fn main() {
    let file = File::open("../inputs/input_3.txt").expect("File not found");
    let buf_reader = BufReader::new(file);

    part_2(buf_reader);
}

fn part_2(reader: BufReader<File>) {
    let sum = reader.lines().chunks(3).into_iter().map(|mut chunk| handle_rucksacks(chunk.next().unwrap().unwrap(), chunk.next().unwrap().unwrap(), chunk.next().unwrap().unwrap())).sum::<i32>();
    println!("{}", sum);
}

fn part_1(reader: BufReader<File>) {
    let sum = reader.lines().map(|str| handle_rucksack(str.unwrap())).sum::<i32>();
    println!("{}", sum);
}

fn handle_rucksack(str: String) -> i32 {
    let halves = str.split_at(str.len() / 2);
    let duplicate = find_duplicate(halves.0, halves.1);
    calculate_priority(duplicate)
}

fn handle_rucksacks(r1: String, r2: String, r3: String) -> i32 {
    for c in r1.chars() {
        if r2.contains(c) && r3.contains(c) {
            return calculate_priority(c);
        }
    }
    panic!("");
}

fn find_duplicate(first_half: &str, second_half: &str) -> char {
    for c in first_half.chars() {
        if second_half.contains(c) {
            return c;
        }
    }
    panic!("");
}

fn calculate_priority(char: char) -> i32 {
    char.to_ascii_lowercase() as i32 - 'a' as i32 + if char.is_lowercase() { 1 } else { 27 }
}