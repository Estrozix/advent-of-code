use std::fs;

fn solve_part1() {
    let str = fs::read_to_string("input_1.txt").unwrap();

    let data: Vec<_> = str.split([' ', '\n']).filter(|el| *el != "").collect();

    let mut left_list: Vec<i32> = data
        .iter()
        .enumerate()
        .filter(|(x, i)| x % 2 == 0)
        .map(|el| el.1.parse::<i32>().unwrap())
        .collect();

    let mut right_list: Vec<i32> = data
        .iter()
        .enumerate()
        .filter(|(x, i)| x % 2 == 1)
        .map(|el| el.1.parse::<i32>().unwrap())
        .collect();

    left_list.sort();
    right_list.sort();

    let mut distances = Vec::<i32>::new();

    for i in 0..left_list.len() {
        distances.push((left_list[i] - right_list[i]).abs());
    }

    let sum = distances.iter().fold(0, |acc, el| acc + el);

    println!("{:?}", sum);
}

fn solve_part2() {
    let str = fs::read_to_string("input_1.txt").unwrap();

    let data: Vec<_> = str.split([' ', '\n']).filter(|el| *el != "").collect();

    let mut left_list: Vec<i32> = data
        .iter()
        .enumerate()
        .filter(|(x, i)| x % 2 == 0)
        .map(|el| el.1.parse::<i32>().unwrap())
        .collect();

    let mut right_list: Vec<i32> = data
        .iter()
        .enumerate()
        .filter(|(x, i)| x % 2 == 1)
        .map(|el| el.1.parse::<i32>().unwrap())
        .collect();

    let mut similarity_scores = Vec::<i32>::new();

    for i in 0..left_list.len() {
        let mut occurences = 0;
        for j in 0..right_list.len() {
            if right_list[j] == left_list[i] {
                occurences += 1;
            }
        }
        similarity_scores.push(left_list[i] * occurences);
    }

    let sum = similarity_scores.iter().fold(0, |acc, el| acc + el);

    println!("{:?}", sum);
}

fn main() {
    solve_part2();
}
