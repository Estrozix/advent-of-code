use core::num;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", process(input));
}

pub fn process(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut num_correct = 0;
        let mut working_sum = 1;

        let numbers: Vec<_> = line.split_whitespace().skip(2).collect();

        let divider = numbers.iter().position(|x| *x == "|").unwrap();

        let winning_numbers = &numbers[0..divider];
        let my_numbers = &numbers[divider+1..];

        for number in my_numbers {
            if winning_numbers.contains(number) {
                num_correct += 1;
                if num_correct > 1 {
                    working_sum *= 2;
                }
            }
        }

        if num_correct >= 1 {
            sum += working_sum;
        }
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input.txt");

        assert_eq!(process(input), 13);
    }
}
