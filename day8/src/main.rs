fn main() {
    part2();
}

use std::{collections::{HashMap, HashSet}, fs};

fn read_input() -> Vec<Vec<char>> {
    return fs::read_to_string("input.txt").unwrap().lines().map(|line| line.chars().collect()).collect();
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    row: usize,
    col: usize
}

fn calc_diff(start_point: &Point, end_point: &Point) -> Point {
    return Point {
        row: end_point.row.abs_diff(start_point.row), 
        col: end_point.col.abs_diff(start_point.col)
    }
}

fn create_antinode(start_point: &Point, end_point: &Point, width: usize, height: usize) -> Result<Point, String> {
    if start_point == end_point {
        return Err("Same points".to_string());
    }
    let row_diff: isize = (end_point.row as isize - start_point.row as isize).try_into().unwrap();
    let col_diff: isize = (end_point.col as isize - start_point.col as isize).try_into().unwrap();

    let new_row: Result<usize, _> = (row_diff + end_point.row as isize).try_into();
    let new_col: Result<usize, _> = (col_diff + end_point.col as isize).try_into();

    match (new_row, new_col) {
        (Ok(row), Ok(col)) if row < height && col < width => Ok(Point { row, col }),
        _ => Err("Invalid point".to_string()),
    }
}

fn part1() {
    let mut arr = read_input();
    let height = arr.len();
    let width = arr[0].len();
    let mut char_cords: HashMap<char, Vec<Point>> = HashMap::new();

    for (row_i, row) in arr.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if col == &'.' {
                continue;
            } else {
                char_cords.entry(*col).or_insert_with(Vec::new).push(Point { row: row_i, col: col_i });
            }
        }
    }

    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_, points) in char_cords.iter() {
        for (i, start_point) in points.iter().enumerate() {
            for (_, end_point) in points.iter().enumerate() {
                if let Ok(point_result) = create_antinode(start_point, end_point, width, height) {
                    antinodes.insert(point_result);
                }
            }
        }
    }

    for point in antinodes.clone() {
        arr[point.row][point.col] = '#';
    }

    for line in arr {
        println!("{}", line.iter().collect::<String>());
    }
    
    println!("{:?}", antinodes.iter());
    println!("{}", antinodes.len());
}


struct sPoint {
    row: isize,
    col: isize
}

fn find_slope(start_point: &Point, end_point: &Point) -> sPoint {
    return sPoint {
        row: end_point.row as isize - start_point.row as isize,
        col: end_point.col as isize - start_point.col as isize
    }
}

fn generate_points(start_point: &Point, end_point: &Point, height: usize, width: usize) -> HashSet<Point> {
    let slope = find_slope(start_point, end_point);
    let mut points: HashSet<Point> = HashSet::new();

    let mut scale = 1;
    loop {
        let new_row: Result<usize, _> = (slope.row * scale + start_point.row as isize).try_into();
        let new_col: Result<usize, _> = (slope.col * scale + start_point.col as isize).try_into();

        match (new_row, new_col) {
            (Ok(new_row), Ok(new_col)) if new_row < height && new_col < width => {
                points.insert(Point {row: new_row, col: new_col});
            },
            _ => {
                break;
            }
        }

        scale += 1;
    }

    scale = -2;

    loop {
        let new_row: Result<usize, _> = (slope.row * scale + start_point.row as isize).try_into();
        let new_col: Result<usize, _> = (slope.col * scale + start_point.col as isize).try_into();

        match (new_row, new_col) {
            (Ok(new_row), Ok(new_col)) if new_row < height && new_col < width => {
                points.insert(Point {row: new_row, col: new_col});
            },
            _ => {
                break;
            }
        }

        scale -= 1;
    }

    return points;


}

fn part2() {
    let mut arr = read_input();
    let height = arr.len();
    let width = arr[0].len();
    let mut char_cords: HashMap<char, Vec<Point>> = HashMap::new();

    for (row_i, row) in arr.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if col == &'.' {
                continue;
            } else {
                char_cords.entry(*col).or_insert_with(Vec::new).push(Point { row: row_i, col: col_i });
            }
        }
    }

    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_, points) in char_cords.iter() {
        for (i, start_point) in points.iter().enumerate() {
            for (_, end_point) in points.iter().enumerate() {
                if start_point == end_point {
                    continue;
                }
                let points = generate_points(start_point, end_point, width, height);
                antinodes.extend(points.iter().cloned());
            }
        }
    }

    for point in antinodes.clone() {
        arr[point.row][point.col] = '#';
    }

    for line in arr {
        println!("{}", line.iter().collect::<String>());
    }
    
    println!("{:?}", antinodes.iter());
    println!("{}", antinodes.len());
}