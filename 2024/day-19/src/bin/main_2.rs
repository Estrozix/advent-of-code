use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let parts = input.split_once("\n\n").unwrap();

    let available_stripes: Vec<&str> = parts.0.split(',').map(|stripe| stripe.trim()).collect();

    let mut max_length = 0;

    available_stripes.iter().for_each(|stripe| {
        if stripe.len() > max_length {
            max_length = stripe.len();
        }
    });

    let patterns: Vec<&str> = parts.1.lines().map(|pattern| pattern.trim()).collect();

    println!("{:?}", available_stripes);

    let mut num_possible = 0;

    let mut memory: HashMap<String, usize> = HashMap::new();

    for pattern in patterns {
        // println!("Trying to match {}", pattern);
        let mut tree: Vec<usize> = Vec::new();
        let mut visited: Vec<usize> = Vec::new();

        if let Some(result) = search_depth(
            &mut tree,
            &mut visited,
            &available_stripes,
            pattern,
            &mut memory,
        ) {
            println!(
                "####### Pattern {} is possible in {:?} ways ######",
                pattern, result
            );
            num_possible += result;
        } else {
            // println!("Pattern {} is NOT possible!", pattern);
        };
    }

    println!("Num possible patterns: {}", num_possible);
}

fn search_depth(
    tree: &mut Vec<usize>,
    visited: &mut Vec<usize>,
    available: &Vec<&str>,
    pattern: &str,
    memory: &mut HashMap<String, usize>,
) -> Option<usize> {
    // println!("CHecking pattern: {}", pattern);
    let mut depth = 0;
    let mut possible: Vec<Vec<usize>> = Vec::new();
    let mut num_possible = 0;

    if let Some(res) = memory.get(&pattern.to_string()) {
        // println!(
        //     "Pattern {} taken from cache, {} possible ways",
        //     pattern, res
        // );
        return Some(*res);
    }

    loop {
        let new_attempt: usize;

        if let Some(current_depth_visited) = visited.get(depth) {
            new_attempt = current_depth_visited + 1;

            if new_attempt >= available.len() {
                if depth as i32 - 1 >= 0 {
                    tree.pop();
                    visited.remove(depth);
                    depth -= 1;

                    continue;
                } else {
                    break;
                }
            }
        } else {
            new_attempt = 0;
        }

        if depth >= visited.len() {
            visited.push(new_attempt);
        } else {
            visited[depth] = new_attempt;
        }

        // println!("{:?}", visited);
        // println!("Current tree: {:?}", tree);

        let mut last_result: String = tree
            .iter()
            .map(|index| available[*index])
            .collect::<Vec<&str>>()
            .join("");

        if last_result.len() > pattern.len() {
            visited.remove(depth);
            depth -= 1;
            continue;
        }

        let new_result = last_result + available[new_attempt];

        // println!("Trying: {} at depth {}", new_result, depth);
        // println!("Should: {}", pattern);

        if pattern == new_result {
            tree.push(new_attempt);

            if let None = possible.iter().find(|s_row| *s_row == tree) {
                possible.push(tree.to_vec());
                num_possible += 1;
            }

            tree.pop();
        } else if pattern.starts_with(&new_result) {
            // println!("worked!");

            let parts = pattern.split_at(new_result.len());

            if let Some(count) =
                search_depth(&mut Vec::new(), &mut Vec::new(), available, parts.1, memory)
            {
                num_possible += count;
                continue;
            }
        }
    }

    memory.insert(pattern.to_string(), num_possible);

    // println!("Pattern {} possible in {} ways", pattern, num_possible);
    return Some(num_possible);
}
