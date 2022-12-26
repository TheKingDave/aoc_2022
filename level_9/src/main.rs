use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rusttype::{Point, Vector};

fn main() {
    let file = File::open("../inputs/input_9.txt").expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut visited_locations: HashSet<Point<i32>> = HashSet::new();

    // let knot_amount = 2;
    let knot_amount = 9;

    let mut rope = vec![Point {x: 0, y: 0}; knot_amount + 1];

    for line in buf_reader.lines() {
        let (amount, dir) = parse_line(line.unwrap());

        for _ in 0..amount {
            let head = rope[0];
            rope[0] = head + dir;

            for i in 0..rope.len() - 1 {
                let distance = rope[i] - rope[i+1];
                if distance.x > 1 || distance.x < -1 || distance.y > 1 || distance.y < -1 {
                    rope[i+1] = rope[i+1] + unit_vector(&distance);
                }
            }

            visited_locations.insert(*rope.last().unwrap());
        }
    }

    println!("Visited locations: {}", visited_locations.len());
}

fn unit_vector(vec: &Vector<i32>) -> Vector<i32> {
    Vector { x: vec.x.clamp(-1, 1), y: vec.y.clamp(-1, 1) }
}

fn parse_line(str: String) -> (i32, Vector<i32>) {
    let mut direction_map: HashMap<char, Vector<i32>> = HashMap::new();
    direction_map.insert('U', Vector { x: 1, y: 0 });
    direction_map.insert('D', Vector { x: -1, y: 0 });
    direction_map.insert('L', Vector { x: 0, y: -1 });
    direction_map.insert('R', Vector { x: 0, y: 1 });

    let (dir_str, amount_str) = str.split_once(" ").unwrap();
    let dir = direction_map.get(&dir_str.chars().next().unwrap()).unwrap();
    let amount = amount_str.parse::<i32>().unwrap();

    (amount, *dir)
}