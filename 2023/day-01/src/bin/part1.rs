fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

fn process(input: &str) -> i32 {
    let mut sum = 0;
    let mut digits = Vec::<i32>::new();

    for line in input.lines() {
        for c in line.chars() {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap() as i32)
            }
        }
        sum = sum + digits.first().unwrap() * 10 + digits.last().unwrap();
        digits.clear();
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn test_code() {
        let input = include_str!("./test_input1.txt");

        println!("{}", input);

        assert_eq!(process(input), 142);
    }
}