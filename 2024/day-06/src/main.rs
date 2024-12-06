use std::fs::read_to_string;

struct HistoryEntry {
    pos: (i32, i32),
    direction: (i32, i32),
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let map_arr: Vec<Vec<char>> = input
        .split("\n")
        .filter(|row| *row != "")
        .map(|row| row.chars().collect())
        .collect();

    // print_map(&map_arr);

    let mut visited_map: Vec<Vec<i32>> = map_arr.iter().map(|row| vec![0; row.len()]).collect();

    let starting_pos: (i32, i32) = map_arr
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| {
                if *c == '^' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .expect("Starting position not found!");

    solve_part1(&map_arr, starting_pos, &mut visited_map);
    solve_part2(&map_arr, starting_pos, &mut visited_map);
}

fn solve_part2(
    original_map: &Vec<Vec<char>>,
    starting_pos: (i32, i32),
    visited_map: &mut Vec<Vec<i32>>,
) {
    let mut options = 0;

    for y in 0..original_map.len() {
        for x in 0..original_map[y].len() {
            if original_map[y][x] != '.' {
                continue;
            }

            // Performance improvement: We only need to check placements on the visited
            // tiles from part1, since we only place one obstacle, and other placements
            // won't affect the path.
            if visited_map[y][x] == 0 {
                continue;
            }

            let mut history_arr: Vec<HistoryEntry> = Vec::new();
            let mut direction = (0, -1);
            let mut pos = starting_pos.clone();

            let mut map_arr = original_map.clone();
            map_arr[y][x] = '#';

            let mut inside = true;
            let mut stuck = false;

            while inside && !stuck {
                let mut in_front = (pos.0 + direction.0, pos.1 + direction.1);

                if check_if_inside(&map_arr, in_front) {
                    let mut object = map_arr[in_front.1 as usize][in_front.0 as usize];

                    while object == '#' {
                        direction = rotate_dir(direction);
                        in_front = (pos.0 + direction.0, pos.1 + direction.1);
                        object = map_arr[in_front.1 as usize][in_front.0 as usize];

                        if history_arr
                            .iter()
                            .find(|entry| {
                                if entry.pos == pos && entry.direction == direction {
                                    return true;
                                } else {
                                    return false;
                                }
                            })
                            .is_some()
                        {
                            stuck = true;
                        }

                        history_arr.push(HistoryEntry {
                            pos: pos.clone(),
                            direction: direction.clone(),
                        });
                    }
                }

                let new_pos = (pos.0 + direction.0, pos.1 + direction.1);

                inside = check_if_inside(&map_arr, new_pos);

                pos = new_pos;
            }

            if stuck {
                options += 1;
                // println!("Placing obstacle at {:?} stucks the guard!", (x, y));
            }
        }
    }
    println!("Total of {:?} options", options);
}

fn solve_part1(
    map_arr: &Vec<Vec<char>>,
    starting_pos: (i32, i32),
    visited_map: &mut Vec<Vec<i32>>,
) {
    let mut inside = true;

    // Note that (0, -1) is up!
    let mut direction = (0, -1);
    let mut pos = starting_pos.clone();

    while inside {
        visited_map[pos.1 as usize][pos.0 as usize] = 1;

        let mut in_front = (pos.0 + direction.0, pos.1 + direction.1);

        if check_if_inside(&map_arr, in_front) {
            let mut object = map_arr[in_front.1 as usize][in_front.0 as usize];

            while object == '#' {
                direction = rotate_dir(direction);
                in_front = (pos.0 + direction.0, pos.1 + direction.1);
                object = map_arr[in_front.1 as usize][in_front.0 as usize];
            }
        }

        let new_pos = (pos.0 + direction.0, pos.1 + direction.1);

        inside = check_if_inside(&map_arr, new_pos);

        pos = new_pos;
    }

    let visited_count = visited_map.iter().flatten().fold(0, |acc, el| acc + *el);

    println!("Total visited: {:?}", visited_count);
}

fn check_if_inside(map: &Vec<Vec<char>>, location: (i32, i32)) -> bool {
    let mut inside = true;

    if location.0 < 0 || location.0 >= map[0].len() as i32 {
        inside = false;
    }
    if location.1 < 0 || location.1 >= map.len() as i32 {
        inside = false;
    }

    return inside;
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{} ", map[y][x]);
        }
        println!();
    }
}

// Note that the up direction is (0, -1) since (0, 0) is top left
fn rotate_dir(x: (i32, i32)) -> (i32, i32) {
    (-x.1, x.0)
}
