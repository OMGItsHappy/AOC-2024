fn main() {
    part1();
}

use std::{borrow::Borrow, cmp::max, collections::HashSet, fs};

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

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match *self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
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

#[derive(Debug)]
struct Point {
    row: i32,
    col: i32,
    direction: Direction
} 

impl Point {
    fn invalid (&self, height: i32, width: i32) -> bool {
        self.row == -1 || 
        self.row == height ||
        self.col == -1 ||
        self.col == width
    }
}

fn move_in_direction(cords_direction: &Point) -> Point {
    return Point {
        row: cords_direction.row + cords_direction.direction.value().0,
        col: cords_direction.col + cords_direction.direction.value().1,
        direction: cords_direction.direction.clone()
    };
}

fn part1() {
    let array = read_input();

    let width: i32 = array[0].len().try_into().unwrap();
    let height: i32 = array.len().try_into().unwrap();

    let mut row_col_dir = Point { row: -1, col: -1, direction: Direction::North };
    for (row_i, row) in array.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == '^' {
                row_col_dir = Point { row: row_i.try_into().unwrap(), col: col_i.try_into().unwrap(), direction: Direction::North };
            }
        }
    }

    let mut locations_visited: HashSet<(i32, i32)> = HashSet::new();
    locations_visited.insert((row_col_dir.row, row_col_dir.col));
    loop {
        println!("{:?}", row_col_dir);
        let next = move_in_direction(&row_col_dir);
        if next.invalid(height, width) {
            break;
        } else if array[next.row as usize][next.col as usize] == '#' {
            row_col_dir.direction.next();
        } else {
            locations_visited.insert((row_col_dir.row, row_col_dir.col));
            row_col_dir = next;
        }
    }

    println!("{}", locations_visited.len() + 1);
}
