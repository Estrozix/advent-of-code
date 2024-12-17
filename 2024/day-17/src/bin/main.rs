use std::fs::read_to_string;

#[derive(Debug)]
struct Computer<'a> {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    instructions: [&'a str; 8],
    pointer: usize,
}

fn main() {
    let mut computer = Computer {
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
        instructions: ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"],
        pointer: 0,
    };

    let input = read_to_string("input.txt").unwrap();

    let parts = input.split_once("\n\n").unwrap();

    // Set up registers
    parts.0.lines().for_each(|line| {
        let line_parts = line.split_once(':').unwrap();
        let reg_id = line_parts.0.chars().last().unwrap();
        let number = line_parts.1.trim().parse::<i32>().unwrap();

        match reg_id {
            'A' => {
                computer.reg_a = number;
            }
            'B' => {
                computer.reg_b = number;
            }
            'C' => {
                computer.reg_c = number;
            }
            _ => panic!("Should not be possible"),
        }
    });

    let mut program: Vec<usize> = Vec::new();
    // Program
    parts.1.lines().for_each(|line| {
        let op_codes = line.split_once(':').unwrap().1.split(',');
        op_codes.for_each(|op_code_str| {
            program.push(op_code_str.trim().parse::<usize>().unwrap());
        });
    });

    let mut output = String::new();

    while computer.pointer < program.len() {
        let op_code = program[computer.pointer];
        let operand = program[computer.pointer + 1] as i32;

        let temp_output = perform_operation(&mut computer, op_code, operand);
        if temp_output != "" {
            output.push_str(&(temp_output + ","));
        }
    }

    println!("{:?}", computer);
    println!("Output: {}", output.trim_matches(','));
}

fn perform_operation(computer: &mut Computer, opcode: usize, operand: i32) -> String {
    let mut output = String::new();

    match opcode {
        // adv
        0 => {
            let numerator = computer.reg_a;
            let denumerator = 2i32.pow(get_value_from_combo(computer, operand) as u32);

            let result: i32 = numerator / denumerator;

            computer.reg_a = result;
            computer.pointer += 2;
        }
        // bxl
        1 => {
            let a = computer.reg_b;
            let b = operand;

            let result: i32 = a ^ b;

            computer.reg_b = result;
            computer.pointer += 2;
        }
        // bst
        2 => {
            let a = get_value_from_combo(computer, operand);
            let result = a % 8;

            computer.reg_b = result;
            computer.pointer += 2;
        }
        // jnz
        3 => {
            if computer.reg_a != 0 {
                computer.pointer = operand as usize;
            } else {
                computer.pointer += 2;
            }
        }
        // bxc
        4 => {
            let a = computer.reg_b;
            let b = computer.reg_c;

            let result = a ^ b;

            computer.reg_b = result;
            computer.pointer += 2;
        }
        // out
        5 => {
            let result = get_value_from_combo(computer, operand) % 8;
            output.push_str(&result.to_string());
            computer.pointer += 2;
        }
        // bdv
        6 => {
            let numerator = computer.reg_a;
            let denumerator = 2i32.pow(get_value_from_combo(computer, operand) as u32);

            let result: i32 = numerator / denumerator;

            computer.reg_b = result;
            computer.pointer += 2;
        }
        // cdv
        7 => {
            let numerator = computer.reg_a;
            let denumerator = 2i32.pow(get_value_from_combo(computer, operand) as u32);

            let result: i32 = numerator / denumerator;

            computer.reg_c = result;
            computer.pointer += 2;
        }
        _ => {
            panic!("Invalid opcode!")
        }
    }

    return output;
}

fn get_value_from_combo(computer: &Computer, combo: i32) -> i32 {
    match combo {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => computer.reg_a,
        5 => computer.reg_b,
        6 => computer.reg_c,
        7 => {
            panic!("reserved, not valid")
        }
        _ => {
            panic!("invalid combo")
        }
    }
}
