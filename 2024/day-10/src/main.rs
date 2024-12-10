use std::{fmt::Debug, fs::read_to_string};

#[derive(Clone, Debug)]
struct Pos(i32, i32, u32);

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let data: Vec<Vec<u32>> = input
        .split('\n')
        .filter(|row| *row != "")
        .map(|row| {
            row.chars()
                .map(|ch| ch.to_digit(10).expect("Could not parse digit"))
                .collect()
        })
        .collect();

    print_map(&data);

    let mut to_explore: Vec<Pos> = Vec::new();

    data.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, el)| {
            if *el == 0 {
                to_explore.push(Pos(x as i32, y as i32, 0));
            }
        })
    });

    let mut total_score = 0;
    let mut total_rating = 0;

    to_explore.iter().for_each(|el| {
        let trailhead_score = explore(&data, el);
        println!(
            "Trailhead at {:?} got score: {:?} and rating: {:?}",
            &el, trailhead_score.0, trailhead_score.1
        );

        total_score += trailhead_score.0;
        total_rating += trailhead_score.1;
    });

    println!(
        "Total score: {:?}, Total rating: {:?}",
        total_score, total_rating
    );
}

fn explore(data: &Vec<Vec<u32>>, pos: &Pos) -> (i32, i32) {
    let mut to_explore: Vec<Pos> = Vec::new();

    to_explore.push(pos.clone());

    let mut found_nines: Vec<Pos> = Vec::new();
    let mut found_distinct = 0;

    while to_explore.len() != 0 {
        println!("{:?}", to_explore);
        let current = to_explore.pop().expect("Should be at least one element");

        if current.2 == 9 {
            found_distinct += 1;
            if let None = found_nines.iter().find(|pos| {
                if pos.0 == current.0 && pos.1 == current.1 {
                    return true;
                } else {
                    return false;
                }
            }) {
                found_nines.push(current);
            }
            continue;
        }

        // Check up
        let up = Pos(current.0, current.1 - 1, current.2 + 1);
        if check_next(&data, &up) {
            to_explore.push(up);
        }

        // Check down
        let down = Pos(current.0, current.1 + 1, current.2 + 1);
        if check_next(&data, &down) {
            to_explore.push(down);
        }

        // Check right
        let right = Pos(current.0 + 1, current.1, current.2 + 1);
        if check_next(&data, &right) {
            to_explore.push(right);
        }

        // Check left
        let left = Pos(current.0 - 1, current.1, current.2 + 1);
        if check_next(&data, &left) {
            to_explore.push(left);
        }
    }

    return (found_nines.iter().count() as i32, found_distinct);
}

fn check_next(data: &Vec<Vec<u32>>, check: &Pos) -> bool {
    if in_bounds(&data, &check) {
        if data[check.1 as usize][check.0 as usize] == check.2 {
            return true;
        }
    } else {
        return false;
    }

    return false;
}

fn in_bounds(data: &Vec<Vec<u32>>, pos: &Pos) -> bool {
    if pos.0 < 0 || pos.0 >= data.first().expect("SHould be at least one element").len() as i32 {
        return false;
    }
    if pos.1 < 0 || pos.1 >= data.len() as i32 {
        return false;
    }

    return true;
}

fn print_map(data: &Vec<Vec<impl Debug>>) {
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            print!("{:?}", data[y][x]);
        }
        println!();
    }
}
