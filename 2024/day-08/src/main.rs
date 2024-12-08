use std::fs::read_to_string;

#[derive(Debug)]
struct Antenna {
    pos: (i32, i32),
}

#[derive(Debug)]
struct AntennaGroup {
    antennas: Vec<Antenna>,
    frequency: char,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<char>> = input
        .split('\n')
        .filter(|row| *row != "")
        .map(|row| row.chars().collect())
        .collect();

    let mut antenna_groups: Vec<AntennaGroup> = Vec::new();

    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, el)| {
            if *el != '.' {
                let antenna = Antenna {
                    pos: (x as i32, y as i32),
                };

                if let Some(antenna_group) = antenna_groups
                    .iter_mut()
                    .find(|group| group.frequency == *el)
                {
                    antenna_group.antennas.push(antenna);
                } else {
                    antenna_groups.push(AntennaGroup {
                        antennas: vec![antenna],
                        frequency: el.clone(),
                    });
                }
            }
        })
    });

    solve_part1(&grid, &antenna_groups);
    solve_part2(&grid, &antenna_groups);
}

fn solve_part1(grid: &Vec<Vec<char>>, antenna_groups: &Vec<AntennaGroup>) {
    let mut grid_with_antinodes = grid.clone();

    for group in antenna_groups {
        for antenna1 in &group.antennas {
            for antenna2 in &group.antennas {
                if antenna1.pos == antenna2.pos {
                    continue;
                }

                let mut pos1 = antenna1.pos;
                let pos2 = antenna2.pos;

                let xdiff = pos2.0 - pos1.0;
                let ydiff = pos2.1 - pos1.1;

                let mut valid = true;

                let antinode_pos = (pos1.0 + 2 * xdiff, pos1.1 + 2 * ydiff);

                if antinode_pos.0 < 0 || antinode_pos.0 >= grid[0].len() as i32 {
                    valid = false;
                }
                if antinode_pos.1 < 0 || antinode_pos.1 >= grid.len() as i32 {
                    valid = false;
                }

                if valid {
                    grid_with_antinodes[antinode_pos.1 as usize][antinode_pos.0 as usize] = '#';
                }
            }
        }
    }

    let unique_locations = grid_with_antinodes
        .iter()
        .flatten()
        .filter(|el| **el == '#')
        .count();

    println!("Unique locations part1: {}", unique_locations);
}

fn solve_part2(grid: &Vec<Vec<char>>, antenna_groups: &Vec<AntennaGroup>) {
    let mut grid_with_antinodes = grid.clone();

    for group in antenna_groups {
        for antenna1 in &group.antennas {
            for antenna2 in &group.antennas {
                if antenna1.pos == antenna2.pos {
                    continue;
                }

                let mut pos1 = antenna1.pos;
                let pos2 = antenna2.pos;

                let xdiff = pos2.0 - pos1.0;
                let ydiff = pos2.1 - pos1.1;

                let mut valid = true;

                while valid {
                    let antinode_pos = (pos1.0 + xdiff, pos1.1 + ydiff);

                    if antinode_pos.0 < 0 || antinode_pos.0 >= grid[0].len() as i32 {
                        valid = false;
                    }
                    if antinode_pos.1 < 0 || antinode_pos.1 >= grid.len() as i32 {
                        valid = false;
                    }

                    if valid {
                        grid_with_antinodes[antinode_pos.1 as usize][antinode_pos.0 as usize] = '#';
                    }

                    pos1 = antinode_pos;
                }
            }
        }
    }

    let unique_locations = grid_with_antinodes
        .iter()
        .flatten()
        .filter(|el| **el == '#')
        .count();

    println!("Unique locations part2: {}", unique_locations);
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{} ", grid[y][x]);
        }
        println!();
    }
}
