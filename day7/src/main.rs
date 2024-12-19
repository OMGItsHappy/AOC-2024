fn main() {
    part2();
}

use std::fs;

fn read_input() -> Vec<Vec<i64>> {
    return fs::read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(|line| 
        line.replace(":", "")
        .split(" ")
        .map(|num| 
            num.parse()
            .unwrap()
        ).collect()
    ).collect();
}

fn calc(total: i64, goal: i64, index: usize, array: &Vec<i64>) -> i64 {
    if total == goal {
        return goal;
    } else if index >= array.len() {
        return -1;
    }
    let num = array[index];
    if total % num == 0 {
        let result = calc(total/num, goal, index + 1, array);
        if result == -1 {
            
        } else {
            return result;
        }
    }
    return calc(total - num, goal, index + 1, array);
}

use regex::Regex;

fn strip(total: i64, next_val: i64) -> i64 {
    let str: String = total.to_string().chars().rev().collect();
    let next_val_str: String = next_val.to_string().chars().rev().collect();

    if !str.contains(&next_val_str) {
        return -1;
    }

    let re = Regex::new(&next_val_str).unwrap();
    return re.replace(&str, "").chars().rev().collect::<String>().parse().unwrap_or(-1);    
}

fn calc_with_concat(total: i64, goal: i64, index: usize, array: &Vec<i64>) -> i64 {
    if total == goal && index >= array.len() {
        return goal;
    } else if index >= array.len() {
        return -1;
    }
    let num = array[index];
    let strip_result = strip(total, num);
    if strip_result != -1 {
        let result = calc_with_concat(strip_result, goal, index + 1, array);
        if result == -1 {

        } else {
            return result;
        }
    }
    if total % num == 0 {
        let result = calc_with_concat(total/num, goal, index + 1, array);
        if result == -1 {
            
        } else {
            return result;
        }
    } 
    return calc_with_concat(total - num, goal, index + 1, array);
}

fn part2() {
    let input = read_input();

    let mut sum = 0;

    for mut line in input {
        line.reverse();
        let total = line.pop().unwrap();
        let goal = line.pop().unwrap();
        let index: usize = 0;

        let result = calc_with_concat(total, goal, index, &line);

        if result == goal {
            sum += total;
        }

        println!("{}", calc_with_concat(total, goal, index, &line));
    }

    println!("{}", sum);
}

fn part1() {
    let input = read_input();

    let mut sum = 0;

    for mut line in input {
        line.reverse();
        let mut total = line.pop().unwrap();
        let goal = line.pop().unwrap();
        let index: usize = 0;

        if calc(total, goal, index, &line) == goal {
            sum += total;
        }

        println!("{}", calc(total, goal, index, &line));
    }

    println!("{}", sum);
}