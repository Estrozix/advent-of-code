use std::{
    fmt::{Debug, Display},
    fs::read_to_string,
};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Dir {
    TOP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug)]
struct Region {
    plant_type: char,
    plots: Vec<Plot>,
    perimeter: i32,
    area: i32,
    sides: i32,
}

#[derive(Clone)]
struct Plot {
    plant: char,
    pos: (usize, usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Perimeter {
    dir: Dir,
    pos: (i32, i32),
}

impl Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.plant)
    }
}

fn main() {
    println!("Hello, world!");
    let input = read_to_string("input.txt").unwrap();

    let mut garden: Vec<Vec<Plot>> = input
        .split("\n")
        .filter(|row| *row != "")
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, plot)| Plot {
                    plant: plot,
                    pos: (x, y),
                })
                .collect()
        })
        .collect();

    let mut claimed: Vec<Vec<bool>> = garden
        .iter()
        .map(|row| row.iter().map(|plot| false).collect())
        .collect();

    let mut regions: Vec<Region> = Vec::new();

    garden.iter().for_each(|row| {
        row.iter().for_each(|plot| {
            if claimed[plot.pos.1][plot.pos.0] {
                return;
            }

            let region = explore(&garden, &plot, &mut claimed);
            regions.push(region);
            // Assume new region
        })
    });

    print_map(&garden);
    print_regions(&regions);

    let sum1 = regions
        .iter()
        .fold(0, |acc, region| acc + region.perimeter * region.area);

    let sum2 = regions
        .iter()
        .fold(0, |acc, region| acc + region.sides * region.area);

    println!("Part1 sum is: {}", sum1);
    println!("Part2 sum is: {}", sum2);
}

fn print_regions(regions: &Vec<Region>) {
    regions.iter().for_each(|region| {
        println!("{:?}", region);
    });
}

fn explore(garden: &Vec<Vec<Plot>>, plot: &Plot, claimed: &mut Vec<Vec<bool>>) -> Region {
    let mut to_explore = vec![plot.clone()];
    let mut plots: Vec<Plot> = Vec::new();
    let mut area = 0;
    let mut perimeter: Vec<Perimeter> = Vec::new();
    let plant_type = plot.plant;

    while to_explore.len() > 0 {
        let plot = to_explore.pop().expect("Should be at least one");

        if claimed[plot.pos.1][plot.pos.0] {
            continue;
        }

        if plot.plant == plant_type {
            claimed[plot.pos.1][plot.pos.0] = true;
            plots.push(plot.clone());
            area += 1;
        }

        let positions = vec![
            (plot.pos.0 as i32, plot.pos.1 as i32 - 1),
            (plot.pos.0 as i32, plot.pos.1 as i32 + 1),
            (plot.pos.0 as i32 + 1, plot.pos.1 as i32),
            (plot.pos.0 as i32 - 1, plot.pos.1 as i32),
        ];

        positions
            .iter()
            .enumerate()
            .for_each(|(pos_id, check_pos)| {
                let direction = match pos_id {
                    0 => Dir::DOWN,
                    1 => Dir::TOP,
                    2 => Dir::LEFT,
                    3 => Dir::RIGHT,
                    _ => panic!("SHould never happen!"),
                };

                if let Some(temp) = search_plot(&garden, *check_pos) {
                    if temp.plant == plot.plant {
                        if !claimed[temp.pos.1][temp.pos.0] {
                            to_explore.push(temp.clone());
                        }
                    } else {
                        perimeter.push(Perimeter {
                            pos: *check_pos,
                            dir: direction,
                        })
                    }
                } else {
                    perimeter.push(Perimeter {
                        pos: *check_pos,
                        dir: direction,
                    })
                };
            });
    }

    let mut check_perimeters: Vec<Perimeter> = perimeter
        .iter()
        .enumerate()
        .map(|(i, peri)| peri.clone())
        .collect();

    let mut done = false;

    println!("Checking perimeters {}: ", plot.plant);
    println!("{:?}", check_perimeters);

    let mut to_remove: i32 = -1;
    let mut pos: usize = 0;

    //while !done {
    //    if let Some(perimeter) = check_perimeters.get(pos) {
    //        check_perimeters.iter().for_each(|neigh| {
    //            if to_remove == -1 {
    //                if perimeter.dir == neigh.dir {
    //                    if perimeter.pos.0 == neigh.pos.0
    //                        && (perimeter.pos.1 - neigh.pos.1).abs() == 1
    //                    {
    //                        to_remove = pos as i32;
    //                        pos = 0;
    //                    } else if perimeter.pos.1 == neigh.pos.1
    //                        && (perimeter.pos.0 - neigh.pos.0).abs() == 1
    //                    {
    //                        to_remove = pos as i32;
    //                        pos = 0;
    //                    }
    //                }
    //            }
    //        });

    //        if to_remove != -1 {
    //            println!("Removing: {:?}", to_remove);
    //            check_perimeters.remove(to_remove as usize);
    //            to_remove = -1;
    //        }
    //    };

    //    pos += 1;

    //    if pos >= check_perimeters.len() {
    //        done = true;
    //    }
    //}
    //
    let mut side_groups: Vec<Vec<Perimeter>> = Vec::new();

    println!("##############################3");

    // Find duplicates
    let mut tries = 0;
    while check_perimeters.len() > 0 {
        let peri1 = check_perimeters.pop().expect("Should be there");

        let mut found = false;

        for side_group in &mut side_groups {
            // println!("Checking for {:?} in side group {:?}", peri1, side_group);
            if side_group
                .iter()
                .find(|peri2| {
                    peri1.dir == peri2.dir
                        && (((peri1.pos.1 - peri2.pos.1).abs() == 1 && peri1.pos.0 == peri2.pos.0)
                            || ((peri1.pos.0 - peri2.pos.0).abs() == 1
                                && peri1.pos.1 == peri2.pos.1))
                })
                .is_some()
            {
                side_group.push(peri1.clone());
                found = true;
                break;
            }
        }

        if !found {
            tries += 1;
            if tries > check_perimeters.len() * 1000 {
                tries = 0;
                side_groups.push(vec![peri1.clone()])
            } else {
                check_perimeters.insert(0, peri1.clone());
            }
        }
    }

    println!("SIDE GROUPS:");
    println!("{:?}", side_groups);

    Region {
        plots,
        sides: side_groups.len() as i32,
        perimeter: perimeter.len() as i32,
        area,
        plant_type: plot.plant,
    }
}

fn search_plot(garden: &Vec<Vec<Plot>>, pos: (i32, i32)) -> Option<Plot> {
    if pos.1 < 0 || pos.1 >= garden.len() as i32 {
        return None;
    }
    if pos.0 < 0 || pos.0 >= garden[pos.1 as usize].len() as i32 {
        return None;
    }

    let x = pos.0 as usize;
    let y = pos.1 as usize;

    return Some(garden[y][x].clone());
}

fn print_map<T: Debug>(map: &Vec<Vec<T>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{:?} ", map[y][x]);
        }
        println!();
    }
}
