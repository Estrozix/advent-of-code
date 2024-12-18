use std::{collections::VecDeque, fmt::Debug, fs::read_to_string};

const SPACE_WIDTH: usize = 71;
const SPACE_HEIGHT: usize = 71;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut space = vec![vec![i32::MAX; SPACE_WIDTH]; SPACE_HEIGHT];

    let mut falling_bytes: VecDeque<(usize, usize)> = input
        .lines()
        .map(|line| {
            let coords = line.split_once(',').unwrap();
            return (
                coords.0.parse::<usize>().expect("should parse"),
                coords.1.parse::<usize>().expect("should parse"),
            );
        })
        .collect();

    print_map(&space);

    fall_bytes(&mut space, &mut falling_bytes, 12);

    print_map(&space);

    let mut most_recent_byte: (usize, usize) = (0, 0);

    loop {
        let previous_space = space.clone();

        if let Some(result) = find_path(&mut space, (0, 0), (SPACE_WIDTH - 1, SPACE_HEIGHT - 1)) {
            println!("Shortest path is {}", result);
        } else {
            println!("No path!");
            break;
        };

        space = previous_space.clone();

        if let Some(fallen_byte) = fall_one_byte(&mut space, &mut falling_bytes) {
            most_recent_byte = fallen_byte;
        } else {
            println!("All bytes has fallen, path still possible!");
            break;
        };
    }

    println!("Byte that prevented path: {:?}", most_recent_byte);
}

fn find_smallest_cost(space: &Vec<Vec<i32>>, options: &Vec<(i32, i32)>) -> usize {
    let first_option = options.first().expect("Should be at least one");

    let mut smallest_id = 0;
    let mut smallest_cost = get_element_at(space, *first_option).expect("Should be there");

    for (id, option) in options.iter().enumerate() {
        if let Some(el) = get_element_at(space, *option) {
            if el < smallest_cost {
                smallest_id = id;
                smallest_cost = get_element_at(space, *option).expect("SHOuld be there");
            }
        }
    }

    smallest_id
}

fn find_path(space: &mut Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let mut unvisited: Vec<(i32, i32)> = space
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, el)| {
                if *el != -1 {
                    return Some((x as i32, y as i32));
                } else {
                    return None;
                }
            })
        })
        .flatten()
        .collect();

    space[start.1][start.0] = 0;

    while unvisited.len() > 0 {
        let current_node = unvisited.remove(find_smallest_cost(space, &unvisited));

        if current_node.0 == end.0 as i32 && current_node.1 == end.1 as i32 {
            break;
        }

        if space[current_node.1 as usize][current_node.0 as usize] == i32::MAX {
            return None;
        }

        let new_cost = get_element_at(space, current_node).expect("Should be there") + 1;

        // Check up
        let pos = (current_node.0, current_node.1 - 1);
        if let Some(up) = get_element_at(space, pos) {
            if up > new_cost {
                space[pos.1 as usize][pos.0 as usize] = new_cost;
            }
        }

        // Check right
        let pos = (current_node.0 + 1, current_node.1);
        if let Some(up) = get_element_at(space, pos) {
            if up > new_cost {
                space[pos.1 as usize][pos.0 as usize] = new_cost;
            }
        }

        // Check down
        let pos = (current_node.0, current_node.1 + 1);
        if let Some(up) = get_element_at(space, pos) {
            if up > new_cost {
                space[pos.1 as usize][pos.0 as usize] = new_cost;
            }
        }

        // Check left
        let pos = (current_node.0 - 1, current_node.1);
        if let Some(up) = get_element_at(space, pos) {
            if up > new_cost {
                space[pos.1 as usize][pos.0 as usize] = new_cost;
            }
        }
    }

    return Some(get_element_at(space, (end.0 as i32, end.1 as i32)).expect("should be there"));
}

fn get_element_at(space: &Vec<Vec<i32>>, pos: (i32, i32)) -> Option<i32> {
    if pos.1 >= 0 {
        if pos.0 >= 0 {
            if let Some(row) = space.get(pos.1 as usize) {
                if let Some(el) = row.get(pos.0 as usize) {
                    return Some(*el);
                }
            }
        }
    }

    return None;
}

fn fall_bytes(space: &mut Vec<Vec<i32>>, falling_bytes: &mut VecDeque<(usize, usize)>, num: i32) {
    for i in 0..num {
        if falling_bytes.len() > 0 {
            let byte = falling_bytes.pop_front().expect("Should be there");
            space[byte.1][byte.0] = -1;
        }
    }
}

fn fall_one_byte(
    space: &mut Vec<Vec<i32>>,
    falling_bytes: &mut VecDeque<(usize, usize)>,
) -> Option<(usize, usize)> {
    if falling_bytes.len() > 0 {
        let byte = falling_bytes.pop_front().expect("Should be there");
        space[byte.1][byte.0] = -1;
        return Some(byte);
    } else {
        return None;
    }
}

fn print_map(map: &Vec<Vec<i32>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == i32::MAX {
                print!(". ");
            } else if map[y][x] == -1 {
                print!("# ");
            } else {
                print!("{} ", map[y][x]);
            }
        }
        println!();
    }
    println!();
}
