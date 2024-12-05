fn main() {
   part2();
}

use std::fs;
use std::collections::HashMap;

fn part1() {
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();
    let contents = fs::read_to_string("input.txt").unwrap();
    for line in contents.lines() {
        let content: Vec<&str> = line.split("   ").collect();
        left_side.push(content[0].parse::<i32>().unwrap());
        right_side.push(content[1].parse::<i32>().unwrap());
    }

    left_side.sort();
    right_side.sort();

    let mut total = 0;

    for (left, right) in left_side.iter().zip(right_side.iter()) {
        total += (left-right).abs();
    }

    println!("{}", total);
}

fn part2() {
    let mut left_side = HashMap::new();
    let mut right_side = HashMap::new();
    let contents = fs::read_to_string("input.txt").unwrap();
    for line in contents.lines() {
        let content: Vec<&str> = line.split("   ").collect();
        let left_value = content[0].parse::<i32>().unwrap();
        let right_value = content[1].parse::<i32>().unwrap();
        *left_side.entry(left_value).or_insert(0) += 1;
        *right_side.entry(right_value).or_insert(0) += 1;
    }

    let mut total = 0;

    for (key, val) in left_side.into_iter() {
        total += right_side.get(&key).unwrap_or(&0) * key * val;
    }

    println!("{}", total);
}