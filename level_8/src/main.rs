use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rusttype::{Point, Vector};

fn main() {
    let file = File::open("../inputs/input_8.txt").expect("File not found");
    let buf_reader = BufReader::new(file);

    let trees: Vec<Vec<u8>> = buf_reader.lines().map(|line| convert_line_to_heights(line.unwrap())).collect();

    // part_1(trees);
    part_2(trees);
}

fn part_2(trees: Vec<Vec<u8>>) {
    let mut high_score = 0;

    for x in 0..trees.len() {
        for y in 0..trees[0].len() {
            let up = find_direction_score(&trees, Point{x, y}, Vector{x: -1, y: 0});
            let down = find_direction_score(&trees, Point{x, y}, Vector{x: 1, y: 0});
            let left = find_direction_score(&trees, Point{x, y}, Vector{x: 0, y: -1});
            let right = find_direction_score(&trees, Point{x, y}, Vector{x: 0, y: 1});

            // println!("({}, {}): {} {} {} {}", x, y, up, down, left, right);

            let score = up * down * left * right;
            high_score = max(high_score, score);
        }
    }

    println!("Highscore: {}", high_score);
}

fn find_direction_score(trees: &Vec<Vec<u8>>, start: Point<usize>, direction: Vector<i32>) -> i32 {
    let mut score = 0;

    let tree_height = lookup_tree_height(trees, &start);
    let mut current_point = start;

    loop {
        add_vector_to_point(&mut current_point, &direction);
        if point_out_of_bounds(&current_point, 0, trees.len(), 0, trees[0].len()) {
            break;
        }

        let height = lookup_tree_height(trees, &current_point);
        if height >= tree_height {
            score += 1;
            break;
        }
        score += 1;
    }

    score
}

#[allow(dead_code)]
fn part_1(trees: Vec<Vec<u8>>) {
    let mut visible: Vec<Vec<bool>> = vec![vec![false; trees[0].len()]; trees.len()];

    for y in 1..trees.len() - 1 {
        mark_visible(&trees, &mut visible, Point { x: 0, y }, Vector { x: 1, y: 0 });
        mark_visible(&trees, &mut visible, Point { x: trees.len() - 1, y }, Vector { x: -1, y: 0 });
    }
    for x in 1..trees[0].len() - 1 {
        mark_visible(&trees, &mut visible, Point { x, y: 0 }, Vector { x: 0, y: 1 });
        mark_visible(&trees, &mut visible, Point { x, y: trees[0].len() - 1 }, Vector { x: 0, y: -1 });
    }

    /*for line in visible.iter() {
        println!("{:?}", line.iter().map(|v| if *v {1} else {0}).collect::<Vec<u8>>());
    }*/

    let visible_trees = visible.iter().flatten().filter(|visible| **visible).count();
    let visible_trees_outside = trees.len() * 2 + (trees[0].len() - 2) * 2;
    let total_visible_trees = visible_trees + visible_trees_outside;
    println!("Visible: {}", total_visible_trees);
}

fn convert_line_to_heights(str: String) -> Vec<u8> {
    str.chars().map(|c| c as u8 - '0' as u8).collect()
}

fn mark_visible(trees: &Vec<Vec<u8>>, visible: &mut Vec<Vec<bool>>, start: Point<usize>, direction: Vector<i32>) {
    let mut max_height = lookup_tree_height(trees, &start);
    let mut current_point = start;

    loop {
        add_vector_to_point(&mut current_point, &direction);
        if point_out_of_bounds(&current_point, 1, trees.len() - 1, 1, trees[0].len()) {
            break;
        }

        let tree_height = lookup_tree_height(trees, &current_point);

        if tree_height > max_height {
            max_height = tree_height;
            set_tree_visible(visible, &current_point);
        }
        if max_height == 9 {
            break;
        }
    }
}

fn add_vector_to_point(point: &mut Point<usize>, vector: &Vector<i32>) {
    point.x = ((point.x as i32) + vector.x) as usize;
    point.y = ((point.y as i32) + vector.y) as usize;
}

fn lookup_tree_height(trees: &Vec<Vec<u8>>, point: &Point<usize>) -> u8 {
    trees[point.x][point.y]
}

fn set_tree_visible(visible: &mut Vec<Vec<bool>>, point: &Point<usize>) {
    visible[point.x][point.y] = true;
}

fn point_out_of_bounds(point: &Point<usize>, min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> bool {
    point.x >= max_x || point.y >= max_y || point.x < min_x || point.y < min_y
}