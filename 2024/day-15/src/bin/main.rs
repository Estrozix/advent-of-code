use std::{fmt::Display, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let parts = input.split_once("\n\n").unwrap();

    let mut player: (usize, usize) = (0, 0);

    let mut map: Vec<Vec<char>> = parts
        .0
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, el)| {
                    if el == '@' {
                        player = (x, y);
                        return '.';
                    } else {
                        return el;
                    }
                })
                .collect()
        })
        .collect();

    let sequence = parts.1.chars();

    for mov in sequence {
        match mov {
            '<' => {
                move_player(&mut map, &mut player, (-1, 0));
            }
            '>' => {
                move_player(&mut map, &mut player, (1, 0));
            }
            '^' => {
                move_player(&mut map, &mut player, (0, -1));
            }
            'v' => {
                move_player(&mut map, &mut player, (0, 1));
            }
            _ => {}
        }
    }

    print_map(&map, player);

    let mut result = 0;

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, el)| {
            if *el == 'O' {
                result += 100 * y;
                result += x;
            }
        })
    });

    println!("Result: {}", result);
}

fn move_player(map: &mut Vec<Vec<char>>, player: &mut (usize, usize), direction: (i32, i32)) {
    let in_front_pos = (
        (player.0 as i32 + direction.0) as usize,
        (player.1 as i32 + direction.1) as usize,
    );

    let in_front = map[in_front_pos.1][in_front_pos.0];

    match in_front {
        '#' => {}
        '.' => {
            player.0 = in_front_pos.0;
            player.1 = in_front_pos.1;
        }
        'O' => {
            if move_box(map, in_front_pos, direction) {
                player.0 = in_front_pos.0;
                player.1 = in_front_pos.1;
            }
        }
        _ => {}
    }
}

fn move_box(map: &mut Vec<Vec<char>>, box_entity: (usize, usize), direction: (i32, i32)) -> bool {
    let in_front_pos = (
        (box_entity.0 as i32 + direction.0) as usize,
        (box_entity.1 as i32 + direction.1) as usize,
    );
    let in_front = map
        .get(in_front_pos.1)
        .unwrap()
        .get(in_front_pos.0)
        .unwrap();

    if *in_front == '#' {
        return false;
    }

    if *in_front == '.' {
        map[box_entity.1][box_entity.0] = '.';
        map[in_front_pos.1][in_front_pos.0] = 'O';
        return true;
    }

    if *in_front == 'O' {
        if move_box(map, in_front_pos, direction) {
            if map[in_front_pos.1][in_front_pos.0] == '.' {
                map[box_entity.1][box_entity.0] = '.';
                map[in_front_pos.1][in_front_pos.0] = 'O';
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    return false;
}

fn print_map<T: Display>(map: &Vec<Vec<T>>, player: (usize, usize)) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if player.0 == x && player.1 == y {
                print!("@");
            } else {
                print!("{}", map[y][x]);
            }
        }
        println!();
    }
}
