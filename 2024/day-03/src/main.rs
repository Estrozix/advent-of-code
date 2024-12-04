use std::fs::read_to_string;

use regex::Regex;

#[derive(Debug)]
struct State {
    position: usize,
    enabled: bool,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mut matches: Vec<String> = Vec::new();

    let dos = find_dos(&input);
    let donts = find_donts(&input);

    let mut states: Vec<State> = Vec::new();

    dos.iter().for_each(|el| {
        states.push(State {
            position: el.clone(),
            enabled: true,
        })
    });

    donts.iter().for_each(|el| {
        states.push(State {
            position: el.clone(),
            enabled: false,
        })
    });

    states.sort_by(|a, b| a.position.cmp(&b.position));

    println!("{:?}", states);

    let mut enabled = true;

    for mat in re.find_iter(&input) {
        let start = mat.start();

        states.iter().for_each(|state| {
            if start > state.position {
                enabled = state.enabled;
                return;
            }
        });

        if enabled {
            matches.push(String::from(&input[mat.start()..mat.end()]));
        }
    }

    println!("{:?}", matches);

    let mut sum = 0;

    for mat in matches {
        sum += multiply(&mat);
    }

    println!("Sum = {}", sum);
}

fn find_dos(text: &str) -> Vec<usize> {
    let re = Regex::new(r"do\(\)").unwrap();

    let mut positions: Vec<usize> = Vec::new();

    for mat in re.find_iter(&text) {
        positions.push(mat.start());
    }

    return positions;
}

fn find_donts(text: &str) -> Vec<usize> {
    let re = Regex::new(r"don't\(\)").unwrap();

    let mut positions: Vec<usize> = Vec::new();

    for mat in re.find_iter(&text) {
        positions.push(mat.start());
    }

    return positions;
}

fn multiply(str_op: &str) -> i32 {
    let re = Regex::new(r"\d+,\d+").unwrap();

    let find = re.find(str_op).unwrap();

    let result = String::from(&str_op[find.start()..find.end()]);

    let numbers: Vec<i32> = result
        .split(",")
        .map(|text| text.parse::<i32>().unwrap())
        .collect();

    if numbers.len() == 2 {
        return numbers[0] * numbers[1];
    } else {
        panic!("Non supported multiplication");
    }
}
