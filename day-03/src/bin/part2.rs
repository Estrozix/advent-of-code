pub fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

#[derive(Debug, Clone, Copy)]
pub struct PartNumber {
    number: u32,
    range: (usize, usize),
    row: usize,
}

#[derive(Debug)]
pub struct Gear {
    row: usize,
    column: usize,
}

pub fn process(input: &str) -> u32 {
    let mut part_numbers = Vec::<PartNumber>::new();
    let mut gears = Vec::<Gear>::new();
    let schematic: Vec<String> = input.lines().map(|x| String::from(x)).collect();

    for (row, line) in schematic.iter().enumerate() {
        let mut digits = Vec::<char>::new();
        let mut digit_found = false;
        let mut digit_start = 0;

        for (i, c) in line.chars().enumerate() {
            if c == '*' {
                gears.push(Gear { row: row, column: i })
            }

            if c.is_digit(10) {
                if !digit_found {
                    digit_found = true;
                    digit_start = i;
                }

                digits.push(c);
            } else if digit_found && (!c.is_digit(10)) {
                digit_found = false;
                let digit_end = i - 1;

                if let Ok(number) = digits.iter().collect::<String>().parse::<u32>() {
                    part_numbers.push(PartNumber { number: number, range: (digit_start, digit_end), row: row })
                };
                digits.clear();
            }
        }
        if digits.len() > 0 {
            let digit_end = line.len() - 1;
            if let Ok(number) = digits.iter().collect::<String>().parse::<u32>() {
                part_numbers.push(PartNumber { number: number, range: (digit_start, digit_end), row: row })
            };
            digits.clear();
        }
    }

    let mut sum = 0;

    for gear in gears {
        let mut connected_parts = Vec::<PartNumber>::new();

        for part_number in &part_numbers {
            if (part_number.row as i32 - gear.row as i32).abs() <= 1 {
                for n_x in part_number.range.0..=part_number.range.1 {
                    if (gear.column as i32 - n_x as i32).abs() <= 1 {
                        connected_parts.push(part_number.clone());
                        break;
                    }
                }
            } 
        }

        if connected_parts.len() == 2 {
            sum += connected_parts[0].number * connected_parts[1].number;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn test() {
        let input = include_str!("./test_input2.txt");

        assert_eq!(process(input), 467835);
    }
}