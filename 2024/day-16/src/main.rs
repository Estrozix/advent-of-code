use std::{fs::read_to_string, i32};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Dir {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Debug)]
struct Node {
    pos: (usize, usize),
    dir: Option<Dir>,
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
                    dir: None,
                });
            } else if *el == 0 {
                unvisited_nodes.push(Node {
                    pos: (x, y),
                    dir: Some(Dir::EAST),
                })
            }
        })
    });

    while unvisited_nodes.len() > 0 {
        let smallest_id = get_smallest(&graph, &unvisited_nodes);

        let current_node = unvisited_nodes.remove(smallest_id);

        let current_distance = graph[current_node.pos.1][current_node.pos.0];

        if let Some(dir) = current_node.dir {
            // Go east
            check_neighbor(
                &mut graph,
                &mut unvisited_nodes,
                current_distance + get_cost(dir.clone(), Dir::EAST),
                (current_node.pos.0 + 1, current_node.pos.1),
                Dir::EAST,
            );
            // Go south
            check_neighbor(
                &mut graph,
                &mut unvisited_nodes,
                current_distance + get_cost(dir.clone(), Dir::SOUTH),
                (current_node.pos.0, current_node.pos.1 + 1),
                Dir::SOUTH,
            );

            // Go west
            check_neighbor(
                &mut graph,
                &mut unvisited_nodes,
                current_distance + get_cost(dir.clone(), Dir::WEST),
                (current_node.pos.0 - 1, current_node.pos.1),
                Dir::WEST,
            );

            // Go north
            check_neighbor(
                &mut graph,
                &mut unvisited_nodes,
                current_distance + get_cost(dir.clone(), Dir::NORTH),
                (current_node.pos.0, current_node.pos.1 - 1),
                Dir::NORTH,
            );
        } else {
            panic!("should always have a dir");
        }
    }

    print_graph(&graph);
    println!("Score of E: {}", graph[end.1][end.0]);
}

fn get_cost(c_dir: Dir, t_dir: Dir) -> i32 {
    match c_dir {
        Dir::NORTH => match t_dir {
            Dir::NORTH => 1 + 1000 * 0,
            Dir::EAST => 1 + 1000 * 1,
            Dir::SOUTH => 1 + 1000 * 2,
            Dir::WEST => 1 + 1000 * 1,
        },
        Dir::EAST => match t_dir {
            Dir::NORTH => 1 + 1000 * 1,
            Dir::EAST => 1 + 1000 * 0,
            Dir::SOUTH => 1 + 1000 * 1,
            Dir::WEST => 1 + 1000 * 2,
        },
        Dir::SOUTH => match t_dir {
            Dir::NORTH => 1 + 1000 * 2,
            Dir::EAST => 1 + 1000 * 1,
            Dir::SOUTH => 1 + 1000 * 0,
            Dir::WEST => 1 + 1000 * 1,
        },
        Dir::WEST => match t_dir {
            Dir::NORTH => 1 + 1000 * 1,
            Dir::EAST => 1 + 1000 * 2,
            Dir::SOUTH => 1 + 1000 * 1,
            Dir::WEST => 1 + 1000 * 0,
        },
    }
}

fn check_neighbor(
    graph: &mut Vec<Vec<i32>>,
    unvisited_nodes: &mut Vec<Node>,
    new_distance: i32,
    target_pos: (usize, usize),
    dir: Dir,
) {
    // Go east
    let target = graph[target_pos.1][target_pos.0];

    if target > new_distance {
        graph[target_pos.1][target_pos.0] = new_distance;

        if let Some(check_node) = unvisited_nodes
            .iter_mut()
            .find(|s_node| s_node.pos.1 == target_pos.1 && s_node.pos.0 == target_pos.0)
        {
            check_node.dir = Some(dir);
        }
    }
}

fn get_smallest(graph: &Vec<Vec<i32>>, nodes: &Vec<Node>) -> usize {
    let mut smallest_node = 0;
    let mut smallest = graph[nodes[smallest_node].pos.1][nodes[smallest_node].pos.0];

    for (id, node) in nodes.iter().enumerate() {
        if graph[node.pos.1][node.pos.0] < smallest {
            smallest_node = id;
            smallest = graph[nodes[smallest_node].pos.1][nodes[smallest_node].pos.0];
        }
    }
    return smallest_node;
}

fn print_graph(graph: &Vec<Vec<i32>>) {
    for y in 0..graph.len() {
        for x in 0..graph[y].len() {
            if graph[y][x] == i32::MAX {
                print!("_");
            } else if graph[y][x] == -1 {
                print!("#");
            } else {
                print!("{} ", graph[y][x]);
            }
        }
        println!();
    }
}
