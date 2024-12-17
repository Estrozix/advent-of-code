use std::{
    fs::read_to_string,
    sync::mpsc,
    thread::{self, JoinHandle},
};

#[derive(Debug)]
struct Computer<'a> {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    instructions: [&'a str; 8],
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

    let mut working_numbers: Vec<u64> = Vec::new();

    let (tx, rx) = mpsc::channel();

    let num_threads = 16;

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    let skip: u64 = 0;

    for i in 1..num_threads + 1 {
        let local_tx = tx.clone();
        let temp_program = program.clone();
        let temp_program_string = program_string.clone();
        let thread = thread::spawn(move || {
            let result = run_loops(
                i + skip - 1,
                num_threads,
                default_b,
                default_c,
                temp_program,
                temp_program_string,
                i,
            );
            local_tx.send(result).unwrap();
        });
        threads.push(thread);
    }

    drop(tx);

    for received in rx {
        working_numbers.push(received);
        working_numbers.sort();
        println!("{:?}", working_numbers);
    }

    for thread in threads {
        let _ = thread.join();
    }
}

fn run_loops(
    starting_a: u64,
    skip_a: u64,
    default_b: u64,
    default_c: u64,
    program: Vec<usize>,
    program_string: String,
    thread_number: u64,
) -> u64 {
    let mut default_a = starting_a;

    let mut computer = Computer {
        reg_a: default_a,
        reg_b: default_b,
        reg_c: default_c,
        instructions: ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"],
        pointer: 0,
    };

    loop {
        if default_a % (100000000 + starting_a) == 0 {
            println!("Thread {} testing a = {}", thread_number, default_a);
        }
        let mut output = String::new();

        computer.reg_a = default_a;
        computer.reg_b = default_b;
        computer.reg_c = default_c;
        computer.pointer = 0;

        while computer.pointer < program.len() {
            let op_code = program[computer.pointer];
            let operand = program[computer.pointer + 1] as u64;

            let temp_output = perform_operation(&mut computer, op_code, operand);
            if let Some(number) = temp_output {
                output.push_str(&(number.to_string() + ","));
            }

            if !program_string.starts_with(&output) {
                break;
            }
        }

        let clean_output = output.trim_matches(',');

        if clean_output == program_string {
            break;
        };

        default_a += skip_a;
    }

    return default_a;
}

fn perform_operation(computer: &mut Computer, opcode: usize, operand: u64) -> Option<u64> {
    let mut output = None;

    match opcode {
        // adv
        0 => {
            let numerator = computer.reg_a;
            let denumerator = 2u64.pow(get_value_from_combo(computer, operand) as u32);

            let result: u64 = numerator / denumerator;

            computer.reg_a = result;
            computer.pointer += 2;
        }
        // bxl
        1 => {
            let a = computer.reg_b;
            let b = operand;

            let result: u64 = a ^ b;

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
            output = Some(result);
            computer.pointer += 2;
        }
        // bdv
        6 => {
            let numerator = computer.reg_a;
            let denumerator = 2u64.pow(get_value_from_combo(computer, operand) as u32);

            let result: u64 = numerator / denumerator;

            computer.reg_b = result;
            computer.pointer += 2;
        }
        // cdv
        7 => {
            let numerator = computer.reg_a;
            let denumerator = 2u64.pow(get_value_from_combo(computer, operand) as u32);

            let result: u64 = numerator / denumerator;

            computer.reg_c = result;
            computer.pointer += 2;
        }
        _ => {
            panic!("Invalid opcode!")
        }
    }

    return output;
}

fn get_value_from_combo(computer: &Computer, combo: u64) -> u64 {
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
