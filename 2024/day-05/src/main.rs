use std::fs::read_to_string;

fn main() {
    solve_part2();
}

fn solve_part2() {
    let input = read_to_string("input.txt").unwrap();

    let split: Vec<&str> = input.split("\n\n").collect();

    let rule_str = split.get(0).unwrap();
    let page_numbers = split.get(1).unwrap();

    let rules: Vec<(i32, i32)> = rule_str
        .split("\n")
        .map(|row| {
            let parts: Vec<i32> = row
                .split("|")
                .map(|part| part.parse::<i32>().unwrap())
                .collect();
            return (*parts.get(0).unwrap(), *parts.get(1).unwrap());
        })
        .collect();

    let rows: Vec<Vec<i32>> = page_numbers
        .split("\n")
        .filter(|row| *row != "")
        .map(|row| {
            row.split(",")
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;

    rows.iter().for_each(|row| {
        if let Some(new_row) = check_and_swap(&rules, &row) {
            let middle = new_row.get(new_row.len() / 2).unwrap();
            sum += middle;

            if !check_row(&rules, &new_row) {
                panic!("Swapped row was not valid!");
            }

            println!(
                "Row: {:?} was invalid, swapped to {:?} valid! Middle is: {:?}",
                row, new_row, middle
            );
        } else {
            println!("Row: {:?} is already valid", row);
        }
    });

    println!("Sum is {:?}", sum);
}

#[allow(dead_code)]
fn solve_part1() {
    let input = read_to_string("input.txt").unwrap();

    let split: Vec<&str> = input.split("\n\n").collect();

    let rule_str = split.get(0).unwrap();
    let page_numbers = split.get(1).unwrap();

    let rules: Vec<(i32, i32)> = rule_str
        .split("\n")
        .map(|row| {
            let parts: Vec<i32> = row
                .split("|")
                .map(|part| part.parse::<i32>().unwrap())
                .collect();
            return (*parts.get(0).unwrap(), *parts.get(1).unwrap());
        })
        .collect();

    let rows: Vec<Vec<i32>> = page_numbers
        .split("\n")
        .filter(|row| *row != "")
        .map(|row| {
            row.split(",")
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;

    rows.iter().for_each(|row| {
        if check_row(&rules, &row) {
            let middle = row.get(row.len() / 2).unwrap();
            sum += middle;
            println!("Row: {:?} is valid! Middle is: {:?}", row, middle);
        } else {
            println!("Row: {:?} is NOT valid", row);
        }
    });

    println!("Sum is {:?}", sum);
}

fn check_and_swap(rules: &Vec<(i32, i32)>, row: &Vec<i32>) -> Option<Vec<i32>> {
    // println!("Checking row: {:?}", row);
    let mut valid = true;

    let mut working_row = row.clone();

    while !check_row(rules, &working_row) {
        rules.iter().for_each(|rule| {
            let number1 = working_row.iter().position(|row_num| *row_num == rule.0);
            let number2 = working_row.iter().position(|row_num| *row_num == rule.1);

            // println!("Num1: {:?}, Num2: {:?}", number1, number2);

            if let (Some(pos1), Some(pos2)) = (number1, number2) {
                if pos1 > pos2 {
                    valid = false;
                    // println!("Rule {:?} failed", rule);
                    let temp = working_row[pos2].clone();
                    working_row[pos2] = working_row[pos1];
                    working_row[pos1] = temp;
                }
            }
        });
    }

    if valid {
        return None;
    } else {
        return Some(working_row);
    }
}

fn check_row(rules: &Vec<(i32, i32)>, row: &Vec<i32>) -> bool {
    let mut valid = true;
    // println!("Checking row: {:?}", row);

    rules.iter().for_each(|rule| {
        let number1 = row.iter().position(|row_num| *row_num == rule.0);
        let number2 = row.iter().position(|row_num| *row_num == rule.1);

        // println!("Num1: {:?}, Num2: {:?}", number1, number2);

        if let (Some(pos1), Some(pos2)) = (number1, number2) {
            if pos1 > pos2 {
                // println!("Rule {:?} failed", rule);
                valid = false;
            }
        }
    });

    return valid;
}
