use std::fmt::Debug;
use std::fs::read_to_string;

const ROOM_WIDTH: i32 = 101;
const ROOM_HEIGHT: i32 = 103;

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let parts = line.split_once(" ").expect("split on space");
            let pos_strs = parts
                .0
                .split_once("=")
                .expect("split on =")
                .1
                .split_once(",")
                .expect("split on ,");
            let pos = (
                pos_strs.0.parse::<i32>().expect("parse"),
                pos_strs.1.parse::<i32>().expect("parse"),
            );
            let vel_strs = parts
                .1
                .split_once("=")
                .expect("split on =")
                .1
                .split_once(",")
                .expect("split on ,");
            let vel = (
                vel_strs.0.parse::<i32>().expect("parse"),
                vel_strs.1.parse::<i32>().expect("parse"),
            );

            return Robot { pos, vel };
        })
        .collect();

    let mut done = false;

    let mut seconds = 0;

    while !done {
        seconds += 1;
        robots.iter_mut().for_each(|robot| update_robot(robot));

        let mut robot_map: Vec<Vec<i32>> = vec![vec![0; ROOM_WIDTH as usize]; ROOM_HEIGHT as usize];

        robots.iter().for_each(|robot| {
            robot_map[robot.pos.1 as usize][robot.pos.0 as usize] += 1;
        });

        if robot_map.iter().flatten().find(|cell| **cell > 1).is_none() {
            done = true;
        }

        print_map(&robot_map);
    }

    println!("Took {} seconds", seconds);
}

fn solve_part1() {
    let input = read_to_string("input.txt").unwrap();

    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let parts = line.split_once(" ").expect("split on space");
            let pos_strs = parts
                .0
                .split_once("=")
                .expect("split on =")
                .1
                .split_once(",")
                .expect("split on ,");
            let pos = (
                pos_strs.0.parse::<i32>().expect("parse"),
                pos_strs.1.parse::<i32>().expect("parse"),
            );
            let vel_strs = parts
                .1
                .split_once("=")
                .expect("split on =")
                .1
                .split_once(",")
                .expect("split on ,");
            let vel = (
                vel_strs.0.parse::<i32>().expect("parse"),
                vel_strs.1.parse::<i32>().expect("parse"),
            );

            return Robot { pos, vel };
        })
        .collect();

    for i in 0..100 {
        robots.iter_mut().for_each(|robot| update_robot(robot));
    }

    let mut robot_map: Vec<Vec<i32>> = vec![vec![0; ROOM_WIDTH as usize]; ROOM_HEIGHT as usize];

    let mut quadrant_count = (0, 0, 0, 0);

    robots.iter().for_each(|robot| {
        robot_map[robot.pos.1 as usize][robot.pos.0 as usize] += 1;

        if robot.pos.1 > ROOM_HEIGHT / 2 {
            // UPPER
            if robot.pos.0 > ROOM_WIDTH / 2 {
                // RIGHT
                quadrant_count.0 += 1;
            } else if robot.pos.0 < ROOM_WIDTH / 2 {
                // LEFT
                quadrant_count.3 += 1;
            }
        } else if robot.pos.1 < ROOM_HEIGHT / 2 {
            // LOWER
            if robot.pos.0 > ROOM_WIDTH / 2 {
                // RIGHT
                quadrant_count.1 += 1;
            } else if robot.pos.0 < ROOM_WIDTH / 2 {
                // LEFT
                quadrant_count.2 += 1;
            }
        }
    });

    print_map(&robot_map);

    let result = quadrant_count.0 * quadrant_count.1 * quadrant_count.2 * quadrant_count.3;

    println!("Result: {}", result);
}

fn print_map<T: Debug>(map: &Vec<Vec<T>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{:?}", map[y][x]);
        }
        println!();
    }
}

fn update_robot(robot: &mut Robot) {
    let mut new_pos = (robot.pos.0 + robot.vel.0, robot.pos.1 + robot.vel.1);

    if new_pos.1 >= ROOM_HEIGHT {
        new_pos.1 = new_pos.1 - ROOM_HEIGHT;
    } else if new_pos.1 < 0 {
        new_pos.1 = new_pos.1 + ROOM_HEIGHT;
    }
    if new_pos.0 >= ROOM_WIDTH {
        new_pos.0 = new_pos.0 - ROOM_WIDTH;
    } else if new_pos.0 < 0 {
        new_pos.0 = new_pos.0 + ROOM_WIDTH;
    }

    robot.pos = new_pos;
}
