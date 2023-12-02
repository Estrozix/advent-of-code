use std::i32;

pub fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

const COLORS: &[&str] = &["blue", "red", "green"];

pub fn process(input: &str) -> i32 {
    let games: Vec<String> = input.lines().map(|x| String::from(x)).collect();

    let mut sum = 0;

    for game in &games {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        let rounds: Vec<String> = game
            .split(|x| x == ':')
            .skip(1)
            .map(|x| String::from(x.trim_start()))
            .collect::<String>()
            .split(|x| x == ';')
            .map(|x| String::from(x.trim()))
            .collect();

        for round in &rounds {
            let mut colors: Vec<(usize, String)> = Vec::new();

            for color in COLORS {
                let found_colors: Vec<(usize, &str)> = round.match_indices(color).collect();
                found_colors.iter().for_each(|x| colors.push((x.0, String::from(x.1))));
            }

            let numbers: Vec<i32> = round.split_whitespace().map(|x| x.parse::<i32>()).filter(|x| x.is_ok()).map(|x| x.unwrap()).collect();
            
            colors.sort_by(|a, b| a.0.cmp(&b.0));

            let red_cube_index = colors.iter().position(|x| x.1 == "red");
            let green_cube_index = colors.iter().position(|x| x.1 == "green");
            let blue_cube_index = colors.iter().position(|x| x.1 == "blue");

            if red_cube_index.is_some() {
                if numbers[red_cube_index.unwrap()] > red_max {
                    red_max = numbers[red_cube_index.unwrap()];
                }
            }

            if green_cube_index.is_some() {
                if numbers[green_cube_index.unwrap()] > green_max {
                    green_max = numbers[green_cube_index.unwrap()];
                }
            }

            if blue_cube_index.is_some() {
                if numbers[blue_cube_index.unwrap()] > blue_max {
                    blue_max = numbers[blue_cube_index.unwrap()];
                }
            }
        }
        // only 12 red cubes, 13 green cubes, and 14 blue cubes        
        sum += red_max * green_max * blue_max;
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
