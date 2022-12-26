//! Solved part 2 with extra help from [[Rust Programming] Advent of Code 2022 Day 11 - Monkey in the Middle](https://www.youtube.com/watch?v=ev1a1YsbJ34)
//! (product / modulo trick at 33:50)

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: String,
    divisible_by: u64,
    throw_true: usize,
    throw_false: usize,
    inspection_count: u64,
}

fn main() {
    let part_1 = false;

    let file = File::open("../inputs/input_11.txt").expect("File not found");
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    let mut monkeys: Vec<Monkey> = Vec::new();

    while let Some(Ok(_monkey_line)) = lines.next() {
        let items_str = lines.next().unwrap().unwrap().split_off(18);
        let operation = lines.next().unwrap().unwrap().split_off(23);
        let divisible_by = lines.next().unwrap().unwrap().split_off(21).parse().unwrap();
        let throw_true = lines.next().unwrap().unwrap().split_off(29).parse().unwrap();
        let throw_false = lines.next().unwrap().unwrap().split_off(30).parse().unwrap();
        lines.next();

        monkeys.push(Monkey {
            items: parse_items(items_str),
            operation,
            divisible_by,
            throw_true,
            throw_false,
            inspection_count: 0,
        });
    }

    let rounds = if part_1 { 20 } else { 10000 };
    let mod_val = monkeys.iter().map(|m| m.divisible_by).reduce(|a, b| a * b).unwrap();

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];

            let mut decisions: Vec<(u64, usize)> = vec![];

            for item in &monkey.items {
                monkey.inspection_count += 1;
                let after_inspection = apply_operation(&item, &monkey.operation);
                let after_bored = if part_1 { after_inspection / 3 } else { after_inspection % mod_val };

                let throw_to_monkey = if after_bored % monkey.divisible_by == 0 { monkey.throw_true } else { monkey.throw_false };

                decisions.push((after_bored, throw_to_monkey));
            }
            monkey.items.clear();
            drop(monkey);

            for (item, dest_monkey) in decisions {
                monkeys[dest_monkey].items.push(item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspection_count);
    monkeys.reverse();
    let monkey_business = &monkeys[0..2].iter().map(|m| m.inspection_count).reduce(|a, b| a * b).unwrap();

    println!("Monkey business: {}", monkey_business);
}

fn parse_items(str: String) -> Vec<u64> {
    str.split(",").map(|i| i.trim().parse().unwrap()).collect()
}

fn apply_operation(old: &u64, operation: &String) -> u64 {
    let (op, num) = operation.split_once(" ").unwrap();

    let number = if num == "old" { *old } else { num.parse().unwrap() };

    if op == "+" {
        old + number
    } else {
        old * number
    }
}