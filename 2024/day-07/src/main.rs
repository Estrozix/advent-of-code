use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
enum Operation {
    ADD = 0,
    MULTIPLY = 1,
    APPEND = 2,
}

const AVAILABLE_OPERATIONS_PART1: [Operation; 2] = [Operation::ADD, Operation::MULTIPLY];
const AVAILABLE_OPERATIONS_PART2: [Operation; 3] =
    [Operation::ADD, Operation::MULTIPLY, Operation::APPEND];

#[derive(Debug, Clone)]
struct Row {
    value: i64,
    nums: Vec<i64>,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let data: Vec<Row> = input
        .split("\n")
        .filter(|str_row| *str_row != "")
        .map(|str_row| {
            let splits = str_row.split_once(":").expect("Could not find : seperator");
            let nums: Vec<i64> = splits
                .1
                .split(" ")
                .filter(|num_str| *num_str != "")
                .map(|str_num| str_num.parse::<i64>().expect("Could not parse num to i32"))
                .collect();

            Row {
                value: splits
                    .0
                    .parse::<i64>()
                    .expect("Could not parse value to i32"),
                nums,
            }
        })
        .collect();

    solve_part1(&data);
    solve_part2(&data);
}

fn solve_part1(data: &Vec<Row>) {
    let result = data.iter().fold(0, |acc, row| {
        if analyse_row(row, &AVAILABLE_OPERATIONS_PART1) {
            acc + row.value
        } else {
            acc
        }
    });

    println!("{:?}", result);
}

fn solve_part2(data: &Vec<Row>) {
    let result = data.iter().fold(0, |acc, row| {
        if analyse_row(row, &AVAILABLE_OPERATIONS_PART2) {
            acc + row.value
        } else {
            acc
        }
    });

    println!("{:?}", result);
}

fn analyse_row(row: &Row, available: &[Operation]) -> bool {
    println!("Analysing: {:?}", row);
    let num_operations = row.nums.len() - 1;

    let results = generate_combinations(num_operations, &available);

    for res in results {
        let mut curr_val = row.nums[0];
        for i in 0..res.len() {
            match res[i] {
                Operation::ADD => curr_val += row.nums[i + 1],
                Operation::MULTIPLY => curr_val *= row.nums[i + 1],
                Operation::APPEND => {
                    curr_val = (curr_val.to_string() + &row.nums[i + 1].to_string())
                        .parse::<i64>()
                        .expect("Should always be possible to parse");
                }
            }
        }
        if curr_val == row.value {
            return true;
        }
    }

    return false;
}

fn generate_combinations(n: usize, available: &[Operation]) -> Vec<Vec<Operation>> {
    if n == 0 {
        return vec![vec![]];
    }

    let smaller_combinations = generate_combinations(n - 1, available);
    let mut result = Vec::new();

    for combo in smaller_combinations {
        for op in available {
            let mut with_op = combo.clone();
            with_op.push(op.clone());
            result.push(with_op);
        }
    }

    result
}
