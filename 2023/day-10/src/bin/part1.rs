use std::{collections::HashMap, fmt::Display, thread::current};

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    x: usize,
    y: usize,
    distance: i32,
    tile_type: TileType,
    tile_char: char,
    up: bool,
    down: bool,
    right: bool,
    left: bool,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.distance > 0 {
            write!(f, "{}", self.distance)
        } else {
            write!(f, "{}", self.tile_char)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, Clone, Copy)]
pub struct Step {
    from: Tile,
    to: Tile,
}

pub fn process(input: &str) -> i32 {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    // let vikings = HashMap::from([
    //     ("Norway", 25),
    //     ("Denmark", 24),
    //     ("Iceland", 12),
    // ]);

    let tile_types: HashMap<char, TileType> = HashMap::from([
        ('|', TileType::NorthSouth),
        ('-', TileType::EastWest),
        ('L', TileType::NorthEast),
        ('J', TileType::NorthWest),
        ('7', TileType::SouthWest),
        ('F', TileType::SouthEast),
        ('.', TileType::Ground),
        ('S', TileType::Start),
    ]);

    let mut map: Vec<Vec<Tile>> = Vec::new();

    let mut start_tile: Option<Tile> = None;

    input.lines().enumerate().for_each(|(r, x)| {
        let row = x
            .chars()
            .enumerate()
            .map(|(c, ch)| {
                let tile_type = *tile_types.get(&ch).unwrap();
                if tile_type == TileType::Start {
                    start_tile = Some(Tile {
                        distance: -1,
                        tile_type: TileType::Start,
                        x: c,
                        y: r,
                        up: true,
                        down: true,
                        right: true,
                        left: true,
                        tile_char: ch,
                    });
                }
                let up = if [
                    TileType::NorthEast,
                    TileType::NorthSouth,
                    TileType::NorthWest,
                ]
                .contains(&tile_type)
                {
                    true
                } else {
                    false
                };
                let down = if [
                    TileType::SouthEast,
                    TileType::SouthWest,
                    TileType::NorthSouth,
                ]
                .contains(&tile_type)
                {
                    true
                } else {
                    false
                };

                let right = if [TileType::EastWest, TileType::NorthEast, TileType::SouthEast]
                    .contains(&tile_type)
                {
                    true
                } else {
                    false
                };
                let left = if [TileType::EastWest, TileType::NorthWest, TileType::SouthWest]
                    .contains(&tile_type)
                {
                    true
                } else {
                    false
                };
                Tile {
                    distance: -1,
                    tile_type,
                    x: c,
                    y: r,
                    up,
                    down,
                    right,
                    left,
                    tile_char: ch,
                }
            })
            .collect::<Vec<Tile>>();

        map.insert(r, row);
    });

    // println!("{map:#?}");
    // println!("Start Coord: {start_coord:?}");

    let start_tile = start_tile.unwrap();
    map[start_tile.y][start_tile.y].distance = 0;

    let mut to_walk: Vec<Tile> = Vec::new();
    to_walk.push(start_tile);

    while to_walk.len() > 0 {
        let current_tile = to_walk.pop().unwrap();
        let mut possible_steps = get_possible_steps(&map, &current_tile);

        // println!("{possible_steps:?}");

        while possible_steps.len() > 0 {
            let step = possible_steps.pop().unwrap();

            to_walk.push(step.to);

            let old_distance = map[step.to.y][step.to.x].distance;
            let new_distance = map[step.from.y][step.from.x].distance + 1;

            if new_distance < old_distance || old_distance == -1 {
                map[step.to.y][step.to.x].distance = new_distance;
            }

            // print_map(&map);
        }
    }

    let mut max_distance = 0;

    map.iter().for_each(|y| {
        y.iter().for_each(|e| {
            if e.distance > max_distance {
                max_distance = e.distance;
            }
        })
    });

    print_map(&map);

    max_distance
}

pub fn print_map(map: &Vec<Vec<Tile>>) {
    map.iter().for_each(|y| {
        y.iter().for_each(|x| print!("{x}"));
        print!("\n")
    });
}

pub fn get_possible_steps(map: &Vec<Vec<Tile>>, current_tile: &Tile) -> Vec<Step> {
    let mut possible_steps: Vec<Step> = Vec::new();

    let current_tile = current_tile.clone();

    let curr_distance = current_tile.distance;

    if current_tile.y as i32 - 1 >= 0 {
        // Check top
        if let Some(tile) = map
            .get(current_tile.y - 1 as usize)
            .unwrap()
            .get(current_tile.x)
        {
            if tile.down && (tile.distance == -1 || (curr_distance - tile.distance).abs() > 1) {
                possible_steps.push(Step {
                    from: current_tile.clone(),
                    to: tile.clone(),
                });
            }
        };
    }

    if current_tile.y + 1 < map.len() {
        // Check bottom
        if let Some(tile) = map.get(current_tile.y + 1).unwrap().get(current_tile.x) {
            if tile.up && (tile.distance == -1 || (curr_distance - tile.distance).abs() > 1) {
                possible_steps.push(Step {
                    from: current_tile.clone(),
                    to: tile.clone(),
                });
            }
        };
    }

    if current_tile.x + 1 < map.first().unwrap().len() {
        // Check right
        if let Some(tile) = map.get(current_tile.y).unwrap().get(current_tile.x + 1) {
            if tile.left && (tile.distance == -1 || (curr_distance - tile.distance).abs() > 1) {
                possible_steps.push(Step {
                    from: current_tile.clone(),
                    to: tile.clone(),
                });
            }
        };
    }

    if current_tile.x as i32 - 1 >= 0 {
        // Check left
        if let Some(tile) = map
            .get(current_tile.y)
            .unwrap()
            .get(current_tile.x - 1 as usize)
        {
            if tile.right && (tile.distance == -1 || (curr_distance - tile.distance).abs() > 1) {
                possible_steps.push(Step {
                    from: current_tile.clone(),
                    to: tile.clone(),
                });
            }
        };
    }

    possible_steps
}

pub fn get_neighbors(pos: (i32, i32), w: i32, h: i32) {
    todo!("NOT DONE");
    // let li = if pos.0 > 0 { -1 } else { 0 };
    // let ri = if pos.0 < w - 1 { 1 } else { 0 };
    // let ti = if pos.1 > 0 { -1 } else { 0 };
    // let bi = if pos.1 < h - 1 { 1 } else { 0 };

    // println!("m_range: {m_range:?}");
    // println!("n_range: {n_range:?}");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input.txt");

        assert_eq!(process(input), 10)
    }

    #[test]
    pub fn test_neighbors() {
        get_neighbors((0, 0), 1, 1);
    }
}
