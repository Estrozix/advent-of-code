fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

const VALID_NUMBERS: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn process(input: &str) -> i32 {
    let mut sum = 0;
    let mut digits = Vec::<(i32, i32)>::new();

    for line in input.lines() {
        for number in 0..VALID_NUMBERS.len() {
            let indexes: Vec<_> = line.match_indices(VALID_NUMBERS[number]).collect();

            for index in indexes {
                digits.push((index.0 as i32, number as i32 + 1));
            }
        }

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                digits.push((i as i32, c.to_digit(10).unwrap() as i32))
            }
        }

        digits.sort_by(|a, b| a.0.cmp(&b.0));

        let adding = digits.first().unwrap().1 * 10 + digits.last().unwrap().1;

        sum += adding;

        digits.clear();
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn test_code() {
        let input = include_str!("./test_input2.txt");

        println!("{}", input);

        assert_eq!(process(input), 281);
    }
}