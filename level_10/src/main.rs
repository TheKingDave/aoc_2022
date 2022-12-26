use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut important_cycles = vec![20, 60, 100, 140, 180, 220];
    important_cycles.reverse();

    let file = File::open("../inputs/input_10.txt").expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut cycle = 0;
    let mut register_x = 1;
    let mut signal_strength = 0;

    for line_res in buf_reader.lines() {
        let mut line = line_res.unwrap();

        let cycle_jump = if line.starts_with("noop") { 1 } else { 2 };
        cycle += cycle_jump;

        if cycle_jump == 2 {
            draw_screen(cycle - 1, register_x);
        }
        draw_screen(cycle, register_x);

        if let Some(important_cycle) = important_cycles.last() {
            if cycle >= *important_cycle {
                let c = important_cycles.pop().unwrap();
                signal_strength += c * register_x;
            }
        }

        if cycle_jump == 2 {
            let num_str = line.split_off(5);
            let num: i32 = num_str.parse().unwrap();
            register_x += num;
        }
    }

    println!();
    println!("Sum of signal strengths: {}", signal_strength);
}

fn draw_screen(cycle: i32, x: i32) {
    let wrapped_cycle = ((cycle - 1) % 40) + 1;

    if wrapped_cycle == 1 {
        println!();
    }

    let wrapped_cycle = ((cycle - 1) % 40) + 1;
    let lit_up = wrapped_cycle >= x && wrapped_cycle < x + 3;

    print!("{}", if lit_up { "#" } else { " " });
}
