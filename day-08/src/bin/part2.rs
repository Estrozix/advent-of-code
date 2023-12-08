use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

#[derive(Debug, Clone)]
pub struct Node {
    left: String,
    right: String,
}

pub fn process(input: &str) -> i64 {
    let data: Vec<_> = input.split("\n\n").collect();
    let instructions = data[0];
    let nodes: Vec<_> = data[1].split("\n").collect();
    let mut node_map: HashMap<String, Node> = HashMap::new();

    for node in &nodes {
        let info: Vec<_> = node.split("=").map(|x| x.trim()).collect();

        let name = info[0];
        let lr: Vec<_> = info[1].split(",").map(|x| x.trim_matches(&[' ', '(', ')'] as &[char])).collect();

        node_map.insert(String::from(name), Node {
            left: String::from(lr[0]),
            right: String::from(lr[1]),
        });
    }

    let mut current_nodes: Vec<String> = Vec::new();

    for node in node_map.keys() {
        if node.chars().last().unwrap() == 'A' {
            current_nodes.push(String::from(node));
        }
    }

    let mut steps: i64 = 0;
    let mut done: Vec<i64> = vec![0; current_nodes.len()];
    let mut iter = instructions.chars().cycle();

    while done.iter().any(|x| *x == 0) {
        let next_instruction = iter.next().unwrap();

        steps += 1;

        for (i, curr_node) in current_nodes.clone().iter().enumerate() {
            let next_node = node_map.get(curr_node).unwrap();

            if next_instruction == 'L' {
                current_nodes[i] = next_node.left.clone();
            } else {
                current_nodes[i] = next_node.right.clone();
            }

            if current_nodes[i].chars().last().unwrap() == 'Z' {
                done[i] = steps;
            }
        }
    }

    println!("{done:?}");

    let lcm = done.into_iter().reduce(num::integer::lcm).unwrap();

    lcm
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input2.txt");

        assert_eq!(process(input), 6)
    }
}