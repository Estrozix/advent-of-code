#![allow(dead_code)]
use std::{fs::read_to_string, thread, time::Duration};

use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

#[derive(Debug)]
struct Computer {
    reg_a: U256,
    reg_b: U256,
    reg_c: U256,
    pointer: usize,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let parts = input.split_once("\n\n").unwrap();

    let mut program_string = String::new();

    let mut default_a = 0;
    let mut default_b = 0;
    let mut default_c = 0;

    // Set up registers
    parts.0.lines().for_each(|line| {
        let line_parts = line.split_once(':').unwrap();
        let reg_id = line_parts.0.chars().last().unwrap();
        let number = line_parts.1.trim().parse::<u64>().unwrap();

        match reg_id {
            'A' => {
                default_a = number;
            }
            'B' => {
                default_b = number;
            }
            'C' => {
                default_c = number;
            }
            _ => panic!("Should not be possible"),
        }
    });

    let mut program: Vec<usize> = Vec::new();
    // Program
    parts.1.lines().for_each(|line| {
        program_string = line.split_once(':').unwrap().1.trim().to_string();
        let op_codes = program_string.split(',');
        op_codes.for_each(|op_code_str| {
            program.push(op_code_str.trim().parse::<usize>().unwrap());
        });
    });

    let mut a_base_8: Vec<usize> = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let len = program.len();
    let mut work_pos = len - 1;
    let mut attempt = 0;

    let mut span: usize = 1;

    loop {
        let a = get_base_10(&a_base_8);
        if let Some(result) = run_computer(a, &program) {
            println!("{:?} > {:?}", a_base_8, result);

            if result == program {
                break;
            }

            if result[work_pos..] == program[work_pos..] {
                work_pos -= 1;
                attempt = 0;
                span = 1;
                continue;
            }

            if attempt > 7i32.pow(span as u32) {
                span += 1;
            }

            attempt += 1;
            increase(&mut a_base_8, work_pos, work_pos + span);
        }
    }

    println!("{:?}", get_base_10(&a_base_8));
}

fn run_loops(a_start: U256, skip: i32, program: &Vec<usize>) {
    let mut a = a_start;
    loop {
        if let Some(result) = run_computer(a, &program) {
            if result == *program {
                println!("{:?}", a);
                break;
            }
        } else {
            a += U256::from(skip);
        };
    }
}

fn increase(base_8_vec: &mut Vec<usize>, pos: usize, max_pos: usize) {
    let len = base_8_vec.len();
    let working_pos = len - pos - 1;

    let current_value = base_8_vec[working_pos];

    if current_value == 7 {
        base_8_vec[working_pos] = 0;
        if max_pos == 0 || pos + 1 < max_pos {
            increase(base_8_vec, pos + 1, max_pos);
        }
    } else {
        base_8_vec[working_pos] += 1;
    }
}

fn get_base_10(vec: &Vec<usize>) -> U256 {
    let mut rev = vec.clone();
    rev.reverse();

    let mut result: U256 = U256::from(0);

    rev.iter().enumerate().for_each(|(i, num)| {
        let base = U256::from(8);
        let mul = base.pow(U256::from(i));
        result += mul * U256::from(*num);
    });

    return result;
}

fn run_computer(default_a: U256, program: &Vec<usize>) -> Option<Vec<usize>> {
    let mut output: Vec<usize> = Vec::new();

    let mut computer = Computer {
        reg_a: default_a,
        reg_b: U256::from(0),
        reg_c: U256::from(0),
        pointer: 0,
    };

    while computer.pointer < program.len() {
        let op_code = program[computer.pointer];
        let operand = program[computer.pointer + 1] as u64;

        let temp_output = perform_operation(&mut computer, op_code, operand);

        if let Some(number) = temp_output {
            output.push(number);
        }

        // if !program.starts_with(&output) {
        //     return None;
        // }
    }

    return Some(output);
}

fn perform_operation(computer: &mut Computer, opcode: usize, operand: u64) -> Option<usize> {
    let mut output = None;

    match opcode {
        // adv
        0 => {
            let numerator = computer.reg_a;
            let denumerator = U256::from(2).pow(get_value_from_combo(computer, operand));

            let result = numerator / denumerator;

            computer.reg_a = result;
            computer.pointer += 2;
        }
        // bxl
        1 => {
            let a = computer.reg_b;
            let b = operand;

            let result = a ^ U256::from(b);

            computer.reg_b = result;
            computer.pointer += 2;
        }
        // bst
        2 => {
            let a = get_value_from_combo(computer, operand);
            let result = a % 8;

            computer.reg_b = U256::from(result);
            computer.pointer += 2;
        }
        // jnz
        3 => {
            if computer.reg_a != U256::from(0) {
                computer.pointer = operand.try_into().unwrap();
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
            output = Some(result.try_into().unwrap());
            computer.pointer += 2;
        }
        // bdv
        6 => {
            let numerator = computer.reg_a;
            let denumerator = U256::from(2).pow(get_value_from_combo(computer, operand));

            let result = numerator / denumerator;

            computer.reg_b = result;
            computer.pointer += 2;
        }
        // cdv
        7 => {
            let numerator = computer.reg_a;
            let denumerator = U256::from(2).pow(get_value_from_combo(computer, operand));

            let result = numerator / denumerator;

            computer.reg_c = result;
            computer.pointer += 2;
        }
        _ => {
            panic!("Invalid opcode!")
        }
    }

    return output;
}

fn get_value_from_combo(computer: &Computer, combo: u64) -> U256 {
    match combo {
        0 => U256::from(0),
        1 => U256::from(1),
        2 => U256::from(2),
        3 => U256::from(3),
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
