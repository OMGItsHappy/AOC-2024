fn main() {
    part2();
}

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> String
{
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);
    return reader.lines().map(|l| l.unwrap()).collect();
}

fn part1()
{
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let numbers = Regex::new(r"\d+").unwrap();
    let input = read_input();

    let mut total = 0;

    for cap in re.captures_iter(&input) {
        let nums: Vec<i32> = numbers.captures_iter(&cap[0]).map(|c| c[0].parse().unwrap()).collect();
        let num1 = nums[0];
        let num2 = nums[1];
        total += num1*num2;
    }

    println!("{total}");
}

fn part2()
{
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let numbers = Regex::new(r"\d+").unwrap();
    let input = read_input();

    let mut total = 0;
    let mut add = true;

    for cap in re.captures_iter(&input) {
        if &cap[0] == "don't()" {
            add = false;
        }
        else if &cap[0] == "do()" {
            add = true;
        }

        else if add {
            println!("{}", &cap[0]);
            let nums: Vec<i32> = numbers.captures_iter(&cap[0]).map(|c| c[0].parse().unwrap()).collect();
            let num1 = nums[0];
            let num2 = nums[1];
            total += num1*num2;
        }
    }

    println!("{total}");
}