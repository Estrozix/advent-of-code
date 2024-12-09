use std::{fs::read_to_string, ops::Range};

#[derive(Debug, Clone)]
struct Block {
    range: Range<i32>,
    id: String,
}

fn main() {
    println!("Hello, world!");

    let a = 0..10;

    let input = read_to_string("input.txt").unwrap();

    let data: Vec<char> = input.chars().collect();

    let mut memory: Vec<String> = Vec::new();

    let mut sweep_id = 0;
    let mut free_space = false;

    data.iter().for_each(|c| {
        if let Some(number) = c.to_digit(10) {
            for _ in 0..number {
                if !free_space {
                    memory.push(sweep_id.to_string());
                } else {
                    memory.push(".".to_string());
                }
            }
            free_space = !free_space;

            if free_space {
                sweep_id += 1;
            }
        };
    });

    solve_part1(&memory);
    solve_part2(&input);
}

fn solve_part2(input: &String) {
    let data: Vec<char> = input.chars().collect();

    let mut memory: Vec<Block> = Vec::new();

    let mut sweep_id = 0;
    let mut sweep_pointer = 0;
    let mut free_space = false;

    data.iter().for_each(|c| {
        if let Some(number) = c.to_digit(10) {
            if !free_space {
                memory.push(Block {
                    range: sweep_pointer..(sweep_pointer as i32 + number as i32),
                    id: sweep_id.to_string(),
                });
            } else {
                memory.push(Block {
                    range: sweep_pointer..(sweep_pointer as i32 + number as i32),
                    id: ".".to_string(),
                });
            }

            free_space = !free_space;
            sweep_pointer += number as i32;

            if free_space {
                sweep_id += 1;
            }
        };
    });

    let mut w_memory: Vec<Block> = memory.clone();

    let mut id_to_move = w_memory
        .iter()
        .rev()
        .find(|block| block.id != ".")
        .unwrap()
        .id
        .parse::<i32>()
        .expect("Could not find id");

    loop {
        let to_move = w_memory.len()
            - 1
            - w_memory
                .iter()
                .rev()
                .position(|block| block.id != "." && block.id.parse::<i32>().unwrap() == id_to_move)
                .unwrap();

        println!("Trying to move {:?}", id_to_move);

        if let Some(first_free) = w_memory.iter().position(|c| {
            if c.id == "." && c.range.len() >= w_memory[to_move].range.len() {
                return true;
            } else {
                return false;
            }
        }) {
            if first_free < to_move {
                println!("Found space!");
                // MOVE
                let to_move_obj = w_memory[to_move].clone();
                let first_free_obj = w_memory[first_free].clone();

                let length_free = first_free_obj.range.len();
                let length_block = to_move_obj.range.len();

                let left = length_free as i32 - length_block as i32;

                w_memory[first_free] = Block {
                    id: to_move_obj.id.clone(),
                    range: first_free_obj.range.start as i32
                        ..first_free_obj.range.start as i32 + length_block as i32,
                };

                w_memory[to_move] = Block {
                    id: ".".to_string(),
                    range: to_move_obj.range.clone(),
                };

                if length_free > length_block {
                    w_memory.insert(
                        first_free + 1,
                        Block {
                            range: first_free_obj.range.start as i32 + length_block as i32
                                ..(first_free_obj.range.start as i32 + length_block as i32 + left),
                            id: ".".to_string(),
                        },
                    );
                }
            }
        }

        id_to_move -= 1;

        if id_to_move == 0 {
            break;
        }
    }

    // let sum = calculate_check_sum(&w_memory);
    //
    // let mut output_string = String::new();

    // w_memory.iter().for_each(|block| {
    //     for i in block.range.clone() {
    //         output_string.push_str(&block.id);
    //     }
    // });

    let sum = w_memory.iter().fold(0, |acc, el| {
        if el.id != "." {
            let mut sum = 0;
            for i in el.range.clone() {
                sum += i as i64 * el.id.parse::<i64>().unwrap();
            }
            return acc + sum;
        } else {
            return acc;
        }
    });

    println!("{:?}", sum);
}

fn solve_part1(memory: &Vec<String>) {
    let mut w_memory: Vec<String> = memory.clone();

    loop {
        let first_free = w_memory.iter().position(|c| *c == ".").unwrap();
        let last_filled =
            w_memory.len() - 1 - w_memory.iter().rev().position(|c| *c != ".").unwrap();

        if first_free > last_filled {
            break;
        }

        w_memory[first_free] = w_memory[last_filled].clone();
        w_memory[last_filled] = ".".to_string();
    }

    let sum = calculate_check_sum(&w_memory);

    println!("{:?}", sum);
}

fn calculate_check_sum(memory: &Vec<String>) -> i64 {
    return memory.iter().enumerate().fold(0, |acc, (i, el)| {
        if *el != "." {
            acc + i as i64 * el.parse::<i64>().unwrap() as i64
        } else {
            acc
        }
    });
}
