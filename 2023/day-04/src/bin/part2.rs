use core::num;

#[derive(Debug)]
pub struct ScratchCard {
    _id: usize,
    // winning_numbers: Vec<String>,
    // my_numbers: Vec<String>,
    num_correct: i32,
    copies: i32,
}

fn main() {
    let input = include_str!("input.txt");

    println!("{}", process(input));
}

pub fn process(input: &str) -> i32 {
    let mut pile: Vec<ScratchCard> = Vec::new();

    for (id, line) in input.lines().enumerate() {
        let numbers: Vec<_> = line.split_whitespace().skip(2).collect();

        let divider = numbers.iter().position(|x| *x == "|").unwrap();

        let winning_numbers: Vec<_> = numbers[0..divider]
            .iter()
            .map(|x| String::from(*x))
            .collect();
        let my_numbers: Vec<_> = numbers[divider + 1..]
            .iter()
            .map(|x| String::from(*x))
            .collect();

        let num_correct = get_num_correct(&my_numbers, &winning_numbers);

        pile.push(ScratchCard {
            _id: id,
            // winning_numbers,
            // my_numbers,
            copies: 1,
            num_correct,
        });
    }

    let mut num_cards = 0;

    for i in 0..pile.len() {
        for _ in 0..pile[i].copies as usize {
            let num_correct = pile[i].num_correct;

            for c in i+1..=i+num_correct as usize {
                pile[c].copies += 1;
            }
        }
        num_cards += pile[i].copies;
    }

    // println!("{:#?}", pile);

    num_cards
}

fn get_num_correct(numbers: &Vec<String>, winning_numbers: &Vec<String>) -> i32 {
    let mut num_correct = 0;

    for number in numbers {
        if winning_numbers.contains(&number) {
            num_correct += 1;
        }
    }

    num_correct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input2.txt");

        assert_eq!(process(input), 30);
    }
}
