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

pub fn process(input: &str) -> i32 {
    let data: Vec<_> = input.split("\n\n").collect();
    let instructions = data[0];
    let nodes: Vec<_> = data[1].split("\n").collect();
    let mut node_map: HashMap<String, Node> = HashMap::new();

    for node in nodes {
        let info: Vec<_> = node.split("=").map(|x| x.trim()).collect();

        let name = info[0];
        let lr: Vec<_> = info[1].split(",").map(|x| x.trim_matches(&[' ', '(', ')'] as &[char])).collect();

        node_map.insert(String::from(name), Node {
            left: String::from(lr[0]),
            right: String::from(lr[1]),
        });
    }

    let start = String::from("AAA");
    let end = String::from("ZZZ");
    let mut current = start.clone();
    let mut steps: i32 = 0;

    let mut iter = instructions.chars().cycle();

    while current != end {
        let next_instruction = iter.next().unwrap();
        let next_node = node_map.get(&current).unwrap();

        if next_instruction == 'L' {
            current = next_node.left.clone();
        } else {
            current = next_node.right.clone();
        }

        steps += 1;
    }

    steps
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input.txt");

        assert_eq!(process(input), 2)
    }
}