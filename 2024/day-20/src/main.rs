use std::{fs::read_to_string, iter::once};

fn main() {
    let input = read_to_string("test.txt").unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    print_map(&map);

    find_path(&map);
}

fn find_path(map: &Vec<Vec<char>>) {
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;

    let unvisited: Vec<(usize, usize)> = map
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

    println!("Start: {:?}, End: {:?}", start, end);
    println!("{:?}", unvisited);
}

fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}
