fn main() {
    part2();
}

fn read_input() -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    return input.lines().map(|line| line.split_whitespace().map(|val| val.parse().unwrap()).collect()).collect();
} 

fn is_increasing(a: i32, b: i32) -> bool {
    b > a
}


fn normal_difference(a: i32, b: i32) -> bool {
    let val = (a-b).abs();
    return val > 0 && val < 4;
}

fn part1() {
    let vals = read_input();
    
    let mut unsafe_rows = 0;
    
    for row in &vals {
        let increasing = row[1] - row [0] > 0;
        let length = row.len();
        for (i, val) in row[..length - 1].iter().enumerate() {
            if is_increasing(*val, row[i + 1]) != increasing || !normal_difference(*val, row[i + 1]) {
                unsafe_rows += 1;
                break;
            }
        }
    }

    println!("{}", vals.len() - unsafe_rows);
}

fn norm_diff(i: usize, row: &Vec<i32>) -> bool {
    if i == row.len() - 1 {
        return true
    }
    let val = (row[i] - row[i + 1]).abs();
    return val > 0 && val < 4;
}

fn check_order_conformity(i: usize, row: &Vec<i32>) -> bool {
    if i == 0 {
        return is_increasing(row[i], row[i + 1]);
    }
    else if i == row.len() - 1 {
        return is_increasing(row[i - 1], row[i]);
    }
    else {
        return is_increasing(row[i], row[i + 1]) == is_increasing(row[i - 1], row[i]);
    }
}

fn part2() {
    let mut vals = read_input();
    
    let mut unsafe_rows = 0;
    
    for row in &mut vals {
        let mut length = row.len();
        let mut i = 0;
        let mut skipped = false;
        while i < length {
            let val = row[i];
            if !check_order_conformity(i, &row) || !norm_diff(i, &row) {
                if !skipped {
                    row.remove(i + 1);
                    skipped = true;
                    length -= 1;
                    if i == length {
                        i -= 1;
                    }
                    continue;
                }
                else {
                    unsafe_rows += 1;
                    break;
                }
            }

            i += 1;
        }
    }

    println!("{}", vals.len() - unsafe_rows);
}

