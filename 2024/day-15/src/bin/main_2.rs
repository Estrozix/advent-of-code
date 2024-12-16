use std::{fmt::Display, fs::read_to_string, io::Read};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let parts = input.split_once("\n\n").unwrap();

    let mut player: (usize, usize) = (0, 0);

    let original_map: Vec<Vec<char>> = parts
        .0
        .lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(|(x, el)| el).collect())
        .collect();

    let mut map: Vec<Vec<char>> = original_map
        .iter()
        .map(|row| {
            row.iter()
                .map(|el| match el {
                    '#' => vec!['#'; 2],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.'; 2],
                    '@' => vec!['@', '.'],
                    _ => panic!(),
                })
                .flatten()
                .collect()
        })
        .collect();

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, el)| {
            if *el == '@' {
                player = (x, y);
            }
        })
    });

    map[player.1][player.0] = '.';

    print_map(&map, player);

    let sequence = parts.1.chars();

    for mov in sequence {
        //let mut buffer = [0; 1];
        //'outer: loop {
        //    let _ = std::io::stdin().read_exact(&mut buffer).expect("no input");
        //    let mov = buffer[0] as char;

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
            '\n' => {}
            _ => {
                //       break 'outer;
            }
        }
        // let _ = std::io::stdin().read_line(&mut String::new());
        print_map(&map, player);
        //}
    }

    let mut result = 0;

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, el)| {
            if *el == '[' {
                result += 100 * y;
                result += x;
            }
        })
    });

    println!("Result: {}", result);
}

fn move_player(map: &mut Vec<Vec<char>>, player: &mut (usize, usize), direction: (i32, i32)) {
    println!("Trying to move {:?}", direction);
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
        '[' => {
            if move_box(map, in_front_pos, direction, true) {
                player.0 = in_front_pos.0;
                player.1 = in_front_pos.1;
            }
        }
        ']' => {
            if move_box(map, in_front_pos, direction, true) {
                player.0 = in_front_pos.0;
                player.1 = in_front_pos.1;
            }
        }
        _ => {}
    }
}

fn move_box(
    map: &mut Vec<Vec<char>>,
    box_entity: (usize, usize),
    direction: (i32, i32),
    should_move: bool,
) -> bool {
    if map[box_entity.1][box_entity.0] == '[' || map[box_entity.1][box_entity.0] == ']' {
    } else {
        return false;
    }

    let in_front_pos = (
        (box_entity.0 as i32 + direction.0) as usize,
        (box_entity.1 as i32 + direction.1) as usize,
    );

    let in_front = map
        .get(in_front_pos.1)
        .unwrap()
        .get(in_front_pos.0)
        .unwrap()
        .clone();

    if in_front == '#' {
        return false;
    }

    // Pushing side-ways is easy
    if direction.0.abs() > 0 {
        if in_front == '.' {
            map[in_front_pos.1][in_front_pos.0] = map[box_entity.1][box_entity.0];
            map[box_entity.1][box_entity.0] = '.';
            return true;
        }

        if in_front == '[' || in_front == ']' {
            if move_box(map, in_front_pos, direction, true) {
                if map[in_front_pos.1][in_front_pos.0] == '.' {
                    map[in_front_pos.1][in_front_pos.0] = map[box_entity.1][box_entity.0];
                    map[box_entity.1][box_entity.0] = '.';
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        // Pushing up and down is harder
    } else if direction.1.abs() > 0 {
        let second_box: (usize, usize);
        let in_front_pos_2: (usize, usize);
        let in_front_2: char;

        if map[box_entity.1][box_entity.0] == '[' {
            second_box = (box_entity.0 + 1, box_entity.1);
            in_front_pos_2 = (second_box.0, (second_box.1 as i32 + direction.1) as usize);
            in_front_2 = map[in_front_pos_2.1][in_front_pos_2.0];

            if map[second_box.1][second_box.0] != ']' {
                panic!()
            }
        } else if map[box_entity.1][box_entity.0] == ']' {
            second_box = (box_entity.0 - 1, box_entity.1);
            in_front_pos_2 = (second_box.0, (second_box.1 as i32 + direction.1) as usize);
            in_front_2 = map[in_front_pos_2.1][in_front_pos_2.0];

            if map[second_box.1][second_box.0] != '[' {
                panic!()
            }
        } else {
            panic!()
        }

        let mut can_move = false;

        if in_front == '.' && in_front_2 == '.' {
            can_move = true;
        } else {
            if in_front == '#' || in_front_2 == '#' {
                return false;
            }

            let mut current_new_box = false;
            let mut other_new_box = false;

            if in_front == '[' || in_front == ']' {
                current_new_box = true;
            }

            if in_front_2 == '[' || in_front_2 == ']' {
                other_new_box = true;
            }

            if in_front_pos == in_front_pos_2 {
                panic!();
            }

            if current_new_box && other_new_box {
                // Make sure they arent the same
                can_move = true;

                if !move_box(map, in_front_pos, direction, false) {
                    can_move = false;
                }

                if map[in_front_pos_2.1][in_front_pos_2.0] == '['
                    || map[in_front_pos_2.1][in_front_pos_2.0] == ']'
                {
                    if can_move && move_box(map, in_front_pos_2, direction, false) {
                        move_box(map, in_front_pos, direction, true);

                        if map[in_front_pos_2.1][in_front_pos_2.0] == '['
                            || map[in_front_pos_2.1][in_front_pos_2.0] == ']'
                        {
                            move_box(map, in_front_pos_2, direction, true);
                        }
                    } else {
                        can_move = false;
                    }
                }

                println!("Recursive move succeeded, moving original!");
                //  if (in_front_pos.0 < in_front_pos_2.0 && in_front == '[' && in_front_2 == ']')
                //      || (in_front_pos.0 > in_front_pos_2.0 && in_front_2 == '[' && in_front == ']')
                //  {
                //      if move_box(map, in_front_pos, direction) {
                //          println!("Same box!");
                //          can_move = true;
                //      }
                //  } else {
                //  }
            } else if current_new_box {
                if move_box(map, in_front_pos, direction, true) {
                    can_move = true;

                    println!("Recursive move succeeded, moving original!");
                }
            } else if other_new_box {
                if move_box(map, in_front_pos_2, direction, true) {
                    can_move = true;
                    println!("Recursive move succeeded, moving original!");
                }
            }
        }

        if can_move {
            // Move first box
            if should_move {
                map[in_front_pos.1][in_front_pos.0] = map[box_entity.1][box_entity.0];
                map[box_entity.1][box_entity.0] = '.';

                // Move second box
                map[in_front_pos_2.1][in_front_pos_2.0] = map[second_box.1][second_box.0];
                map[second_box.1][second_box.0] = '.';
            }
            return true;
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
