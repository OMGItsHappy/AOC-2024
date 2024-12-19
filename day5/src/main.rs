fn main() {
    part1();
}

use std::{collections::HashMap, fs, io};

fn read_input() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut mappings: HashMap<i32, Vec<i32>>= HashMap::new();
    let mut print_orders: Vec<Vec<i32>> = Vec::new();
    let iter = content.lines().into_iter();
    let mut stage = 1;
    for line in iter {
        if stage == 2 {
            print_orders.push(line.split(",").map(|s| s.parse().unwrap()).collect());
            continue;
        }
        if let Ok(vals) = line.split("|").map(|s| s.parse()).collect::<Result<Vec<i32>, _>>() {
            mappings.entry(vals[0]).or_insert_with(Vec::new).push(vals[1]);
        } else {
            stage = 2;
            continue;
        };
        
    }

    return (mappings, print_orders);
}

fn part1() {
    let result = read_input();
    println!("{:?}", result);
}