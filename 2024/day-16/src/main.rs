use std::{fs::read_to_string, i32, thread::current};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Dir {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Debug, Clone)]
struct Node {
    pos: (usize, usize),
    dir: Option<Dir>,
    cost: i32,
    prevs: Vec<(usize, usize, Dir, i32)>,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut end: (usize, usize) = (0, 0);

    let mut graph: Vec<Vec<i32>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, el)| {
                    if el == '.' || el == 'E' {
                        if el == 'E' {
                            end = (x, y);
                        }
                        return i32::MAX;
                    } else if el == 'S' {
                        return 0;
                    } else {
                        return -1;
                    }
                })
                .collect()
        })
        .collect();

    let mut unvisited_nodes: Vec<Node> = Vec::new();

    graph.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, el)| {
            if *el == i32::MAX {
                unvisited_nodes.push(Node {
                    pos: (x, y),
                    dir: Some(Dir::NORTH),
                    cost: i32::MAX,
                    prevs: Vec::new(),
                });
                unvisited_nodes.push(Node {
                    pos: (x, y),
                    dir: Some(Dir::EAST),
                    cost: i32::MAX,
                    prevs: Vec::new(),
                });
                unvisited_nodes.push(Node {
                    pos: (x, y),
                    dir: Some(Dir::SOUTH),
                    cost: i32::MAX,
                    prevs: Vec::new(),
                });
                unvisited_nodes.push(Node {
                    pos: (x, y),
                    dir: Some(Dir::WEST),
                    cost: i32::MAX,
                    prevs: Vec::new(),
                });
            } else if *el == 0 {
                unvisited_nodes.push(Node {
                    pos: (x, y),
                    dir: Some(Dir::EAST),
                    cost: 0,
                    prevs: Vec::new(),
                });
                unvisited_nodes.push(Node {
                    pos: (x, y),
                    dir: Some(Dir::NORTH),
                    cost: i32::MAX,
                    prevs: Vec::new(),
                });
                unvisited_nodes.push(Node {
                    pos: (x, y),
                    dir: Some(Dir::WEST),
                    cost: i32::MAX,
                    prevs: Vec::new(),
                });
                unvisited_nodes.push(Node {
                    pos: (x, y),
                    dir: Some(Dir::SOUTH),
                    cost: i32::MAX,
                    prevs: Vec::new(),
                });
            }
        })
    });

    let mut result = i32::MAX;
    let mut resulting_dir: Dir = Dir::EAST;

    let mut all_nodes = unvisited_nodes.clone();

    while unvisited_nodes.len() > 0 {
        let smallest_id = get_smallest(&unvisited_nodes);

        let current_node = unvisited_nodes.remove(smallest_id);

        if current_node.pos == end {
            if current_node.cost < result {
                result = current_node.cost;
                resulting_dir = current_node.dir.clone().expect("Should be");
            }
        }

        let current_distance = current_node.cost;

        if let Some(dir) = current_node.clone().dir {
            // Go forward
            let delta: (i32, i32) = match dir {
                Dir::NORTH => (0, -1),
                Dir::EAST => (1, 0),
                Dir::SOUTH => (0, 1),
                Dir::WEST => (-1, 0),
            };

            check_neighbor(
                &mut unvisited_nodes,
                current_distance + 1,
                (
                    (current_node.pos.0 as i32 + delta.0) as usize,
                    (current_node.pos.1 as i32 + delta.1) as usize,
                ),
                &current_node,
                &mut all_nodes,
                dir.clone(),
            );

            // Rotate North
            check_neighbor(
                &mut unvisited_nodes,
                current_distance + get_cost(dir.clone(), Dir::NORTH),
                (current_node.pos.0, current_node.pos.1),
                &current_node,
                &mut all_nodes,
                Dir::NORTH,
            );
            // Rotate East
            check_neighbor(
                &mut unvisited_nodes,
                current_distance + get_cost(dir.clone(), Dir::EAST),
                (current_node.pos.0, current_node.pos.1),
                &current_node,
                &mut all_nodes,
                Dir::EAST,
            );
            // Rotate South
            check_neighbor(
                &mut unvisited_nodes,
                current_distance + get_cost(dir.clone(), Dir::SOUTH),
                (current_node.pos.0, current_node.pos.1),
                &current_node,
                &mut all_nodes,
                Dir::SOUTH,
            );
            // Rotate West
            check_neighbor(
                &mut unvisited_nodes,
                current_distance + get_cost(dir.clone(), Dir::WEST),
                (current_node.pos.0, current_node.pos.1),
                &current_node,
                &mut all_nodes,
                Dir::WEST,
            );
        } else {
            panic!("should always have a dir");
        }
    }

    println!("Score of E: {}", result);

    let end_node = all_nodes
        .iter()
        .find(|s_node| {
            if s_node.pos == end && s_node.dir.clone().unwrap() == resulting_dir {
                true
            } else {
                false
            }
        })
        .expect("Should be there");

    let mut on_path: Vec<(usize, usize)> = Vec::new();

    walk(
        &all_nodes,
        (
            end_node.pos.0,
            end_node.pos.1,
            end_node.dir.clone().expect("Sholud bet htere"),
            end_node.cost,
        ),
        &mut on_path,
    );

    on_path.sort();
    on_path.dedup();

    for bench in &on_path {
        graph[bench.1][bench.0] = -2;
    }

    print_graph(&graph);

    println!("Benches: {:?}", on_path.len());
}

fn walk(all_nodes: &Vec<Node>, node: (usize, usize, Dir, i32), on_path: &mut Vec<(usize, usize)>) {
    on_path.push((node.0, node.1));
    if let Some(next_node) = all_nodes.iter().find(|s_node| {
        if s_node.pos == (node.0, node.1) && s_node.dir.clone().unwrap() == node.2.clone() {
            true
        } else {
            false
        }
    }) {
        next_node.prevs.iter().for_each(|new_node| {
            walk(all_nodes, new_node.clone(), on_path);
        });
    };
}

fn get_cost(c_dir: Dir, t_dir: Dir) -> i32 {
    match c_dir {
        Dir::NORTH => match t_dir {
            Dir::NORTH => 1000 * 0,
            Dir::EAST => 1000 * 1,
            Dir::SOUTH => 1000 * 2,
            Dir::WEST => 1000 * 1,
        },
        Dir::EAST => match t_dir {
            Dir::NORTH => 1000 * 1,
            Dir::EAST => 1000 * 0,
            Dir::SOUTH => 1000 * 1,
            Dir::WEST => 1000 * 2,
        },
        Dir::SOUTH => match t_dir {
            Dir::NORTH => 1000 * 2,
            Dir::EAST => 1000 * 1,
            Dir::SOUTH => 1000 * 0,
            Dir::WEST => 1000 * 1,
        },
        Dir::WEST => match t_dir {
            Dir::NORTH => 1000 * 1,
            Dir::EAST => 1000 * 2,
            Dir::SOUTH => 1000 * 1,
            Dir::WEST => 1000 * 0,
        },
    }
}

fn check_neighbor(
    unvisited_nodes: &mut Vec<Node>,
    new_distance: i32,
    target_pos: (usize, usize),
    current_node: &Node,
    all_nodes: &mut Vec<Node>,
    dir: Dir,
) {
    if let Some(target) = unvisited_nodes
        .iter_mut()
        .find(|s_node| s_node.pos == target_pos && dir == s_node.dir.clone().unwrap())
    {
        if target.cost >= new_distance {
            if let Some(all_target) = all_nodes.iter_mut().find(|s_node| {
                s_node.pos == target.pos
                    && target.dir.clone().unwrap() == s_node.dir.clone().unwrap()
            }) {
                if target.cost > new_distance {
                    all_target.prevs = vec![(
                        current_node.pos.0,
                        current_node.pos.1,
                        current_node.dir.clone().expect("should be"),
                        current_node.cost,
                    )]
                } else if target.cost == new_distance {
                    all_target.prevs.push((
                        current_node.pos.0,
                        current_node.pos.1,
                        current_node.dir.clone().expect("should be"),
                        current_node.cost,
                    ));
                }

                if target.cost > new_distance {
                    target.cost = new_distance;
                }
            } else {
                panic!("Should always find some");
            }
        }
    };
}

fn get_smallest(nodes: &Vec<Node>) -> usize {
    let mut smallest = nodes.first().unwrap().cost;
    let mut id_smallest = 0;

    for (i, node) in nodes.iter().enumerate() {
        if node.cost < smallest {
            smallest = node.cost;
            id_smallest = i;
        }
    }

    return id_smallest;
}

fn print_graph(graph: &Vec<Vec<i32>>) {
    for y in 0..graph.len() {
        for x in 0..graph[y].len() {
            if graph[y][x] == i32::MAX {
                print!(" ");
            } else if graph[y][x] == -1 {
                print!("#");
            } else if graph[y][x] == -2 {
                print!("-");
            } else {
                print!("{} ", graph[y][x]);
            }
        }
        println!();
    }
}
