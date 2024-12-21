use std::fs::read_to_string;

fn main() {
    let input = read_to_string("test.txt").unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    print_map(&map);

    find_path(&map);
}

fn find_path(map: &Vec<Vec<char>>) {
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;

    let cost_map: Vec<Vec<i32>> = map
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

    while unvisited.len() > 0 {
        let smallest_id = get_smallest(&cost_map, &unvisited);
        let current_pos = unvisited[smallest_id];

        if cost_map[current_pos.1][current_pos.0] == i32::MAX {
            break;
        }
    }

    println!("Start: {:?}, End: {:?}", start, end);
    println!("{:?}", unvisited);
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
