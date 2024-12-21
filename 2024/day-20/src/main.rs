use std::fs::read_to_string;

fn main() {
    let input = read_to_string("test.txt").unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    print_map(&map);

    find_path(&map);
}

fn find_path(map: &Vec<Vec<char>>) -> Option<i32> {
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;

    let mut cost_map: Vec<Vec<i32>> = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|el| match *el {
                    '#' => -1,
                    _ => i32::MAX,
                })
                .collect()
        })
        .collect();

    let mut unvisited: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, el)| match *el {
                    '.' => Some((x, y)),
                    'S' => {
                        start = Some((x, y));
                        Some((x, y))
                    }
                    'E' => {
                        end = Some((x, y));
                        Some((x, y))
                    }
                    _ => None,
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    if let Some(start) = start {
        cost_map[start.1][start.0] = 0;
    } else {
        panic!("Didnt find start!");
    }

    if let None = end {
        panic!("Didnt find end");
    }

    let mut end_cost: Option<i32> = None;

    while unvisited.len() > 0 {
        let smallest_id = get_smallest(&cost_map, &unvisited);
        let current_pos = unvisited.remove(smallest_id);
        let current_cost = cost_map[current_pos.1][current_pos.0];

        if current_cost == i32::MAX {
            println!("Only inaccessible tiles left, breaking!");
            break;
        }

        if let Some(end) = end {
            if end == current_pos {
                println!("Found solution!");
                end_cost = Some(current_cost);
                break;
            }
        }

        // TODO: Check all directions
        // Right
        let pos: (i32, i32) = (current_pos.0 as i32 + 1, current_pos.1 as i32);
        if current_cost + 1 < check_pos(&cost_map, pos) {
            cost_map[pos.1 as usize][pos.0 as usize] = current_cost + 1;
        }

        // Down
        let pos: (i32, i32) = (current_pos.0 as i32, current_pos.1 as i32 + 1);
        if current_cost + 1 < check_pos(&cost_map, pos) {
            cost_map[pos.1 as usize][pos.0 as usize] = current_cost + 1;
        }

        // Left
        let pos: (i32, i32) = (current_pos.0 as i32 - 1, current_pos.1 as i32);
        if current_cost + 1 < check_pos(&cost_map, pos) {
            cost_map[pos.1 as usize][pos.0 as usize] = current_cost + 1;
        }

        // Up
        let pos: (i32, i32) = (current_pos.0 as i32, current_pos.1 as i32 - 1);
        if current_cost + 1 < check_pos(&cost_map, pos) {
            cost_map[pos.1 as usize][pos.0 as usize] = current_cost + 1;
        }
    }

    if let Some(end_cost) = end_cost {
        println!("Found path, smallest cost is {}", end_cost);
        return Some(end_cost);
    } else {
        return None;
    }
}

fn check_pos(cost_map: &Vec<Vec<i32>>, pos: (i32, i32)) -> i32 {
    if pos.0 < 0 || pos.1 < 0 {
        return -1;
    }

    return cost_map[pos.1 as usize][pos.0 as usize];
}

fn get_smallest(map: &Vec<Vec<i32>>, unvisited: &Vec<(usize, usize)>) -> usize {
    let mut smallest_id: usize = 0;
    let smallest_pos = unvisited[smallest_id];
    let mut smallest_value = map[smallest_pos.1][smallest_pos.0];

    for (i, node) in unvisited.iter().enumerate().skip(1) {
        if map[node.1][node.0] < smallest_value {
            smallest_id = i;
            smallest_value = map[node.1][node.0];
        }
    }

    return smallest_id;
}

fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}
