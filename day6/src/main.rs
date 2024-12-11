fn main() {
    part1();
}

use std::{cmp::max, collections::HashSet, fs};

fn read_input() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("input.txt").unwrap();
    return contents.lines().map(|s| s.chars().collect()).collect();
}

fn generate_cords(row_start: i32, row_end: i32, col_start: i32, col_end: i32) -> Result<Vec<(i32, i32)>, String> {
    let row_diff = row_end-row_start;
    let col_diff = col_end-col_start;
    let row_sign = if row_diff != 0 {
        row_diff/row_diff.abs()
    } else {
        0
    };
    let col_sign = if col_diff != 0 {
        col_diff/col_diff.abs()
    } else {
        0
    };

    if row_diff != 0 && col_diff != 0 && row_diff.abs() != col_diff.abs() {
        return Err("The line must be diagonal.".to_string());
    } else {
        return Ok((0..max(row_diff.abs(), col_diff.abs()).abs())
                .into_iter()
                .map(|offset| offset * row_sign + row_start)
            .zip((0..max(row_diff.abs(), col_diff.abs()))
                .into_iter()
                .map(|offset| offset * col_sign + col_start))
            .collect());
    }
}
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match *self {
            Direction::North => (1, 0),
            Direction::South => (-1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn next(&mut self) {
        *self = match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn part1() {
    let array = read_input();

    let width = array[0].len().try_into().unwrap();
    let height = array.len().try_into().unwrap();

    let mut row_col_dir: (i32, i32, Direction) = (-1, -1, Direction::North);

    let mut locations: Vec<Vec<i32>> = Vec::new();
    for (row_i, row) in array.iter().enumerate() {
        let mut tmp: Vec<i32> = Vec::new();
        for (col_i, col) in row.iter().enumerate() {
            if col == &'#' {
                tmp.push(col_i.try_into().unwrap());
            } else if *col == '^' {
                row_col_dir = (row_i.try_into().unwrap(), col_i.try_into().unwrap(), Direction::North);
            }
        }
        locations.push(tmp);
    }

    fn add_dir()

    let mut locations_visited: HashSet<(i32, i32)> = HashSet::new();
    while row_col_dir.0 != -1 && 
        row_col_dir.0 != height &&
        row_col_dir.1 != -1 &&
        row_col_dir.1 != width {
            locations_visited.insert((row_col_dir.0, row_col_dir.1));
            row_col_dir.0 += row_col_dir.2.value().0;
            row_col_dir.1 += row_col_dir.2.value().1;
            
        }

    println!("{:?}", generate_cords(10, 0, 5, 14));
}
