use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");

    let input = read_to_string("input.txt").unwrap();

    let map: Vec<Vec<char>> = input
        .split("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .filter(|array| array.len() > 0)
        .collect::<Vec<Vec<char>>>();

    print_map(&map);
    let found = search_map_mas(&map);

    println!("Found {} XMAS", found);
}

fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{:?}", map[y][x]);
        }
        println!()
    }
}

fn print_map_i32(map: &Vec<Vec<i32>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{:?},", map[y][x]);
        }
        println!()
    }
}

fn search_map(map: &Vec<Vec<char>>) -> i32 {
    let found_map: Vec<Vec<i32>> = Vec::new();

    let mut found = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'X' {
                println!("Found x at ({:?}, {:?})", x, y);
                found += check_word(map, (x as i32, y as i32));
            }
        }
    }

    found
}

fn search_map_mas(map: &Vec<Vec<char>>) -> i32 {
    let mut found_a = vec![vec![0; map[0].len()]; map.len()];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'M' {
                println!("Found x at ({:?}, {:?})", x, y);
                check_mas_word(map, (x as i32, y as i32))
                    .iter()
                    .for_each(|pos| found_a[pos.1 as usize][pos.0 as usize] += 1);
            }
        }
    }

    print_map_i32(&found_a);

    let found_num = found_a
        .iter()
        .flatten()
        .fold(0, |acc, x| if *x >= 2 { acc + 1 } else { acc });

    println!("Found: {}", found_num);

    return found_num;
}

fn check_mas_word(map: &Vec<Vec<char>>, loc: (i32, i32)) -> Vec<(i32, i32)> {
    let mut locations: Vec<(i32, i32)> = Vec::new();
    let search: Vec<char> = vec!['A', 'S'];
    // Search up + right
    if recursive_search(map, loc, (1, -1), &search) {
        locations.push((loc.0 + 1, loc.1 - 1));
    }

    // Search up + left
    if recursive_search(map, loc, (-1, -1), &search) {
        locations.push((loc.0 - 1, loc.1 - 1));
    }

    // Search down + right
    if recursive_search(map, loc, (1, 1), &search) {
        locations.push((loc.0 + 1, loc.1 + 1));
    }

    // Search down + left
    if recursive_search(map, loc, (-1, 1), &search) {
        locations.push((loc.0 - 1, loc.1 + 1));
    }

    locations
}

fn check_word(map: &Vec<Vec<char>>, loc: (i32, i32)) -> i32 {
    let search: Vec<char> = vec!['M', 'A', 'S'];

    let mut found: i32 = 0;
    // Search right
    if recursive_search(map, loc, (1, 0), &search) {
        found += 1;
    }

    // Search left
    if recursive_search(map, loc, (-1, 0), &search) {
        found += 1;
    }

    // Search up
    if recursive_search(map, loc, (0, -1), &search) {
        found += 1;
    }

    // Search down
    if recursive_search(map, loc, (0, 1), &search) {
        found += 1;
    }

    // Search up + right
    if recursive_search(map, loc, (1, -1), &search) {
        found += 1;
    }

    // Search up + left
    if recursive_search(map, loc, (-1, -1), &search) {
        found += 1;
    }

    // Search down + right
    if recursive_search(map, loc, (1, 1), &search) {
        found += 1;
    }

    // Search down + left
    if recursive_search(map, loc, (-1, 1), &search) {
        found += 1;
    }

    return found;
}

fn recursive_search(
    map: &Vec<Vec<char>>,
    loc: (i32, i32),
    update: (i32, i32),
    search_string: &Vec<char>,
) -> bool {
    let mut search = search_string.clone();

    println!("Search string: {:?}", search);

    if search.len() == 0 {
        return true;
    }

    if check_pos(map, (loc.0 + update.0, loc.1 + update.1), search[0])
        .ok()
        .is_some()
    {
        search.remove(0);
        return recursive_search(map, (loc.0 + update.0, loc.1 + update.1), update, &search);
    } else {
        return false;
    }
}

fn check_pos(map: &Vec<Vec<char>>, loc: (i32, i32), c: char) -> Result<(), &'static str> {
    if loc.1 < 0 || loc.1 as usize >= map.len() {
        return Err("Out of bounds!");
    }
    if loc.0 < 0 || loc.0 as usize >= map[loc.1 as usize].len() {
        return Err("Out of bounds!");
    }

    if map[loc.1 as usize][loc.0 as usize] != c {
        return Err("Wrong character!");
    }

    Ok(())
}
