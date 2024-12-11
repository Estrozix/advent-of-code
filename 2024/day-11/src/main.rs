use std::{collections::HashMap, fs::read_to_string, time};

fn main() {
    let timer = time::Instant::now();

    let input = read_to_string("input.txt").unwrap();

    let initial_stones: Vec<u32> = input
        .split(" ")
        .map(|stone| {
            stone
                .replace("\n", "")
                .parse::<u32>()
                .expect("Shouild parse")
        })
        .collect();

    let mut memory = HashMap::new();

    let mut count: u64 = 0;

    initial_stones.iter().for_each(|stone| {
        count += count_splits(*stone, 75, &mut memory);
    });

    println!("Total of {:?} stones", count + initial_stones.len() as u64);

    println!("Took {:.2?}", timer.elapsed());
}

fn count_splits(stone: u32, depth: u32, memory: &mut HashMap<(u32, u32), u64>) -> u64 {
    let mut value = stone;
    let mut splits = 0;

    if memory.contains_key(&(stone, depth)) {
        return *memory.get(&(stone, depth)).expect("Already checked");
    }

    for i in 0..depth {
        let value_str = value.to_string();

        if value == 0 {
            value = 1;
        } else if value_str.len() % 2 == 0 {
            splits += 1;

            let parts = value_str.split_at(value_str.len() / 2);
            value = parts.0.parse::<u32>().expect("parse");

            splits += count_splits(
                parts.1.parse::<u32>().expect("parse"),
                depth - i - 1,
                memory,
            );
        } else {
            value = value * 2024;
        }
    }

    memory.insert((stone, depth), splits);
    return splits;
}

// fn explore_stone(stone: u32, depth: i32) -> u32 {
//     if depth > 74 {
//         return 1;
//     }
//
//     if stone == 0 {
//         return explore_stone(1, depth + 1);
//     }
//
//     let len = stone.checked_ilog10().unwrap_or(0) + 1;
//     if len % 2 == 0 {
//         let stone_str = stone.to_string();
//         let parts = stone_str.split_at(stone_str.len() / 2);
//         return explore_stone(parts.0.parse::<u32>().expect("should parse"), depth + 1)
//             + explore_stone(parts.1.parse::<u32>().expect("should parse"), depth + 1);
//     }
//
//     return explore_stone(stone * 2024, depth + 1);
// }
//
// fn explore_stone_old(original_stones: &Vec<u64>) -> Vec<u64> {
//     let mut stones: Vec<u64> = Vec::new();
//
//     original_stones.iter().for_each(|stone| {
//         let stone_str = stone.to_string();
//         let len = stone_str.len();
//
//         if *stone == 0 {
//             stones.push(1);
//         } else if len % 2 == 0 {
//             let parts = stone_str.split_at(len / 2);
//             stones.push(parts.0.parse::<u64>().expect("should parse"));
//             stones.push(parts.1.parse::<u64>().expect("should parse"));
//         } else {
//             stones.push(stone * 2024);
//         }
//     });
//
//     return stones;
// }
