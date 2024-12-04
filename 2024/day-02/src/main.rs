use std::{fs::read_to_string, string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let rows: Vec<&str> = input.split('\n').filter(|row| *row != "").collect();

    let num_safe = rows.iter().filter(|row| allow_one_bad(row)).count();

    println!("{:?}", num_safe);
}

fn allow_one_bad(row: &str) -> bool {
    let original_numbers: Vec<i32> = row
        .split(' ')
        .map(|row| row.parse::<i32>().unwrap())
        .collect();

    let safe = determine_if_safe(row);

    if safe {
        return true;
    }

    for i in 0..original_numbers.len() {
        let place = i;
        let input = row
            .split(" ")
            .enumerate()
            .filter(|(i, c)| *i != place)
            .map(|(i, c)| c.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let temp_safe = determine_if_safe(&input);

        if temp_safe {
            return true;
        }
    }

    return false;
}

fn determine_if_safe(row: &str) -> bool {
    let numbers: Vec<i32> = row
        .split(' ')
        .map(|row| row.parse::<i32>().unwrap())
        .collect();

    let mut last: i32 = 0;
    let mut type_mode: i32 = -10;
    let mut safe = true;

    numbers.iter().enumerate().for_each(|(i, num)| {
        if i != 0 {
            let diff = num - last;

            if diff.abs() < 1 || diff.abs() > 3 {
                safe = false;
                return;
            }

            if type_mode == -10 {
                type_mode = diff.signum();
            } else {
                if type_mode != diff.signum() {
                    safe = false;
                    return;
                }
            }
        }

        last = num.clone();
    });

    println!("Row {:?} is {:?}", row, safe);

    return safe;
}
