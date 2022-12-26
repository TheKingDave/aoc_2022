use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("../inputs/input_7.txt").expect("File not found");
    let buf_reader = BufReader::new(file);

    let mut directories: HashMap<String, i64> = HashMap::new();
    let mut current_dir: Vec<String> = vec!["/".to_string()];

    for line_res in buf_reader.lines() {
        let mut line = line_res.unwrap();

        if line.starts_with("$ cd ") {
            let dir = line.split_off(5);
            if dir == "/" {
                current_dir.clear();
                current_dir.push("/".to_string());
            } else if dir == ".." {
                current_dir.pop();
            } else {
                current_dir.push(dir);
            }
        } else if line.starts_with("$ ls") {
            // Ignore ls
        } else if line.starts_with("dir ") {
            // Is a directory
            // Ignore, dir is created in file
        } else {
            // Is a file
            let (size_str, _name) = line.split_once(" ").unwrap();
            let size = size_str.parse::<i64>().unwrap();

            let mut dir_name: String = "".to_string();
            for dir in &current_dir {
                dir_name.push_str(dir);
                *directories.entry(dir_name.to_owned()).or_default() += size;
            }
        }
    }

    // part_1(directories);
    part_2(directories);
}

#[allow(dead_code)]
fn part_1(mut directories: HashMap<String, i64>) {
    directories.retain(|_key, value| value <= &mut (100000 as i64));
    println!("{:?}", directories.values().sum::<i64>());
}

#[allow(dead_code)]
fn part_2(directories: HashMap<String, i64>) {
    let filesystem_size: i64 = 70000000;
    let free_space_needed: i64 = 30000000;

    let used_space = directories.get("/").unwrap();
    let currently_free_space = filesystem_size - used_space;
    let free_up_space = free_space_needed - currently_free_space;

    let mut big_enough_dirs: Vec<&i64> = directories.values().filter(|size| size >= &&free_up_space).collect();
    big_enough_dirs.sort();

    println!("{:?}", big_enough_dirs[0]);
}