fn main() {
    part1();
}

use std::collections::HashSet;
use std::fs;

fn read_input() -> Vec<Vec<char>>{
    let contents = fs::read_to_string("input.txt").unwrap();
    let value: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    return value;
}

fn index_conversion(indexs: [i32; 2], len: usize) -> Result<[usize; 2], String> {
    let mut values: [usize; 2] = [0, 0];
    for (i, &index) in indexs.iter().enumerate() {
        let tmp_index: usize = if index >= 0 {
            index.try_into().map_err(|_| format!("Invalid index: {}", index))?
        } else {
            (len as i32 - index).try_into().map_err(|_| format!("Invalid index: {}", index))?
        };

        if tmp_index >= len {
            return Err(format!("Invalid index: {}", index));
        }

        values[i] = tmp_index;
    }
    
    return Ok(values);
}

fn part1() {
    let mut hash_set: HashSet<String> = HashSet::new();
    let input = read_input();
    let len = input[0].len();
    let mut row_arr: [usize; 4] = [0; 4];
    let mut col_arr: [usize; 4] = [0; 4];
    for (row_i, row) in input.iter().enumerate() {
        for col in 0..row.len() {
            for side in 0..3 {
                'sign_loop: for sign in -1..2 as i32 {
                    let mut word = String::new();
                    for scale in 0..4 {
                        let mut index = [scale, scale];
                        if side == 2 && sign == 0 {
                            continue 'sign_loop;
                        }
                        if side == 2 {
                            index = [index[0] * sign, index[1] * sign];
                        }
                        else {
                            index[side] *= sign;
                        }

                        index[0] += row_i as i32;
                        index[1] += col as i32;

                        let indicies = index_conversion(index, len);
                        if indicies.is_err() {
                            continue 'sign_loop;
                        }

                        let indicies_unwrapped = indicies.unwrap();
                        row_arr[scale as usize] = indicies_unwrapped[0];
                        col_arr[scale as usize] = indicies_unwrapped[1];
                        word.push(input[indicies_unwrapped[0]][indicies_unwrapped[1]]);
                    }

                    if word == "SAMX" {
                        row_arr.reverse();
                        col_arr.reverse();
                    }
                    let string: String = row_arr.iter().chain(col_arr.iter()).map(|&num| num.to_string()).collect::<Vec<String>>().join(",");
                    if !hash_set.contains(&string) && word == "XMAS" || word == "SAMX" {
                        hash_set.insert(string);
                    }
                }
            }
        }
    }

    println!("{}", hash_set.len());
    
}
