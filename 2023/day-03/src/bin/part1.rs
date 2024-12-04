pub fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

#[derive(Debug)]
pub struct PartNumber {
    number: u32,
    range: (usize, usize),
    row: usize,
}

pub fn process(input: &str) -> u32 {

    let symbols = ['*', '$', '#', '+', '=', '/', '%', '@', '&', '-'];

    let mut part_numbers = Vec::<PartNumber>::new();
    let schematic: Vec<String> = input.lines().map(|x| String::from(x)).collect();

    for (row, line) in schematic.iter().enumerate() {
        let mut digits = Vec::<char>::new();
        let mut digit_found = false;
        let mut digit_start = 0;
        let mut digit_end = 0;

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if !digit_found {
                    digit_found = true;
                    digit_start = i;
                }

                digits.push(c);
            } else if digit_found && (!c.is_digit(10)) {
                digit_found = false;
                digit_end = i - 1;

                if let Ok(number) = digits.iter().collect::<String>().parse::<u32>() {
                    part_numbers.push(PartNumber { number: number, range: (digit_start, digit_end), row: row })
                };
                digits.clear();
            }
        }
        if digits.len() > 0 {
            digit_end = line.len() - 1;
            if let Ok(number) = digits.iter().collect::<String>().parse::<u32>() {
                part_numbers.push(PartNumber { number: number, range: (digit_start, digit_end), row: row })
            };
            digits.clear();
        }
    }

    // println!("{:?}", part_numbers);

    let mut sum = 0;
    let max_height = schematic.len();
    let max_width = schematic[0].len();
    let mut not_part = Vec::<PartNumber>::new();

    for part_number in part_numbers {
        let mut found_symbol = false;

        let li = std::cmp::max((part_number.range.0 as i32) - 1, 0) as usize;
        let ri = std::cmp::min((part_number.range.1 as i32) + 1, max_width as i32 - 1) as usize;

        if (part_number.row as i32) - 1 >= 0 {
            schematic[part_number.row - 1][li..=ri].chars().for_each(|x| if symbols.contains(&x) { found_symbol = true });
        }
        if (part_number.row as i32) + 1 < max_height as i32 {
            schematic[part_number.row + 1][li..=ri].chars().for_each(|x| if symbols.contains(&x) { found_symbol = true });
        }

        schematic[part_number.row][li..=ri].chars().for_each(|x| if symbols.contains(&x) { found_symbol = true });

        if found_symbol {
            sum += part_number.number
        } else {
            not_part.push(part_number);
        }
    }

    //println!("{:?}", not_part);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn test() {
        let input = include_str!("./test_input1.txt");

        assert_eq!(process(input), 4361);
    }
}