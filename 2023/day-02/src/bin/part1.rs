use std::i32;

pub fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

const COLORS: &[&str] = &["blue", "red", "green"];

pub fn process(input: &str) -> i32 {
    let games: Vec<String> = input.lines().map(|x| String::from(x)).collect();

    let mut sum = 0;

    let mut game_id = 0;

    for game in &games {
        game_id += 1;

        let rounds: Vec<String> = game
            .split(|x| x == ':')
            .skip(1)
            .map(|x| String::from(x.trim_start()))
            .collect::<String>()
            .split(|x| x == ';')
            .map(|x| String::from(x.trim()))
            .collect();

        let number_of_rounds = rounds.len();
        let mut num_ok_rounds = 0;

        for round in &rounds {
            let mut colors: Vec<(usize, String)> = Vec::new();

            for color in COLORS {
                let found_colors: Vec<(usize, &str)> = round.match_indices(color).collect();
                found_colors.iter().for_each(|x| colors.push((x.0, String::from(x.1))));
            }

            let numbers: Vec<i32> = round.split_whitespace().map(|x| x.parse::<i32>()).filter(|x| x.is_ok()).map(|x| x.unwrap()).collect();

            // print!("{:?}", numbers);

            colors.sort_by(|a, b| a.0.cmp(&b.0));

            let red_cube_index = colors.iter().position(|x| x.1 == "red");
            let green_cube_index = colors.iter().position(|x| x.1 == "green");
            let blue_cube_index = colors.iter().position(|x| x.1 == "blue");

            let mut red_ok = true;
            let mut green_ok = true;
            let mut blue_ok = true;

            if red_cube_index.is_some() {
                if numbers[red_cube_index.unwrap()] > 12 {
                    red_ok = false;
                }
            }

            if green_cube_index.is_some() {
                if numbers[green_cube_index.unwrap()] > 13 {
                    green_ok = false;
                }
            }

            if blue_cube_index.is_some() {
                if numbers[blue_cube_index.unwrap()] > 14 {
                    blue_ok = false;
                }
            }

            if red_ok && green_ok && blue_ok {
                num_ok_rounds += 1;
            }
        }
        // only 12 red cubes, 13 green cubes, and 14 blue cubes
        if num_ok_rounds == number_of_rounds {
            sum += game_id;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = include_str!("test_input1.txt");

        assert_eq!(process(input), 8);
    }
}
