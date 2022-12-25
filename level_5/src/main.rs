use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let test = false;
    let part1 = false;

    let file_path = if test { "../inputs/input_5_test.txt" } else { "../inputs/input_5.txt" };
    let create_count = if test { 3 } else { 9 };

    let file = File::open(file_path).expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut lines = buf_reader.lines();
    let mut stack_lines: Vec<String> = lines.by_ref().take_while(|line| !line.as_ref().unwrap().starts_with(" 1")).map(|l| l.unwrap()).collect();
    stack_lines.reverse();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; create_count];

    for stack_line in stack_lines {
        stack_line
            .chars()
            .enumerate()
            .filter(|&(i, _)| (i + 3) % 4 == 0)
            .map(|(_, v)| v)
            .enumerate()
            .for_each(|(i, c)| if c != ' ' { stacks[i].push(c) });
    }

    let move_lines: Vec<String> = lines.skip(1).map(|l| l.unwrap()).collect();

    for move_line in move_lines {
        let (count, from, to) = parse_move_line(move_line);

        if part1 {
            for _ in 0..count {
                let char = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(char);
            }
        } else {
            let len = stacks[from - 1].len();
            let removed = stacks[from - 1].split_off(len-count);
            stacks[to - 1].extend(removed);
        }
    }

    let out = stacks.iter().map(|s| s.last().unwrap()).map(|c| c.to_string()).collect::<Vec<String>>().join("");
    println!("Solution: {}", out);
}

// count, from, to
fn parse_move_line(str: String) -> (usize, usize, usize) {
    let mut splits = str.split(" ");

    splits.next();
    let count = splits.next().unwrap().parse::<usize>().unwrap();
    splits.next();
    let from = splits.next().unwrap().parse::<usize>().unwrap();
    splits.next();
    let to = splits.next().unwrap().parse::<usize>().unwrap();

    (count, from, to)
}