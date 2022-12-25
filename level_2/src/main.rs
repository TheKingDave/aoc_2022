use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../inputs/input_2.txt").expect("File not found");
    let buf_reader = BufReader::new(file);

    part_2(buf_reader);
}

// Returns 0, 1, 2 for the letters
fn get_plays(str: String) -> (i32, i32) {
    let opponent = str.chars().nth(0).unwrap() as i32 - 'A' as i32;
    let own = str.chars().nth(2).unwrap() as i32 - 'X' as i32;
    return (opponent, own);
}

// Play: 1, 2, 3
// outcome: 0: lose, 1: draw, 2: win
fn calculate_points(play: i32, outcome: i32) -> i32 {
    return play + outcome * 3;
}

#[allow(dead_code)]
fn part_1(reader: BufReader<File>) {
    let mut points = 0;

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let plays = get_plays(line);
        let outcome = (plays.1 - plays.0 + 1).rem_euclid(3);
        points += calculate_points(plays.1 + 1, outcome);
    }

    println!("{:?}", points);
}

#[allow(dead_code)]
fn part_2(reader: BufReader<File>) {
    let mut points = 0;

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let plays = get_plays(line);
        let play = (plays.0 + plays.1 - 1).rem_euclid(3);
        points += calculate_points(play + 1, plays.1);
    }

    println!("{:?}", points);
}