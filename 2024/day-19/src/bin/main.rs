use std::fs::read_to_string;

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

    for pattern in patterns {
        // println!("Trying to match {}", pattern);
        let mut tree: Vec<usize> = Vec::new();
        let mut visited: Vec<usize> = Vec::new();

        if let Some(result) = search_depth(
            &mut tree,
            &mut visited,
            &available_stripes,
            pattern,
            max_length,
        ) {
            println!("Pattern {} is possible using {:?}", pattern, result);
            num_possible += 1;
        } else {
            println!("Pattern {} is NOT possible!", pattern);
        };
    }

    println!("Num possible patterns: {}", num_possible);
}

fn search_depth(
    tree: &mut Vec<usize>,
    visited: &mut Vec<usize>,
    available: &Vec<&str>,
    pattern: &str,
    max_length: usize,
) -> Option<Vec<usize>> {
    let mut depth = 0;
    let mut max_depth = 0;

    loop {
        let new_attempt: usize;

        if let Some(current_depth_visited) = visited.get(depth) {
            new_attempt = current_depth_visited + 1;

            if new_attempt >= available.len() {
                if depth as i32 - 1 >= 0 {
                    tree.pop();
                    visited.remove(depth);
                    depth -= 1;

                    // If we need to go further back than the maximum length of stripes
                    // then it's impossible
                    if (max_depth as i32 - depth as i32).abs() > max_length as i32 {
                        return None;
                    }

                    continue;
                } else {
                    return None;
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
            return Some(tree.to_vec());
        } else if pattern.starts_with(&new_result) {
            // println!("worked!");
            tree.push(new_attempt);
            depth += 1;

            if depth > max_depth {
                max_depth = depth;
            }
        }
    }
}
