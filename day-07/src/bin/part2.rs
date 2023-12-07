use std::{
    cmp::{min, Ordering},
    collections::HashMap,
};

#[derive(Clone, Copy)]
pub enum WinType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Hand {
    cards: String,
    bet: i32,
    counts: Option<HashMap<char, i32>>,
    card_points: Option<Vec<i32>>,
}

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

const CARDS_TYPES: &[char] = &[
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

pub fn process(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| String::from(x))
                .collect::<Vec<_>>()
        })
        .map(|x| Hand {
            cards: String::from(x.first().unwrap()),
            bet: x.last().unwrap().parse::<i32>().unwrap(),
            counts: Some(
                x.first()
                    .unwrap()
                    .chars()
                    .fold(HashMap::new(), |mut map, val| {
                        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                        map
                    }),
            ),
            card_points: Some(
                x.first()
                    .unwrap()
                    .chars()
                    .map(|c| CARDS_TYPES.iter().position(|p| *p == c).unwrap() as i32 + 1)
                    .collect(),
            ),
        })
        .collect();

    hands.sort_by(|a, b| compare(a, b));

    let mut answer = 0;

    for (i, hand) in hands.iter().enumerate() {
        answer += hand.bet as usize * (i + 1);
    }

    answer
}

fn get_win_type(a: &Hand) -> WinType {
    let count_map = a.counts.clone().unwrap();

    let num_jokers = *count_map.get(&'J').unwrap_or(&0);

    let counts: Vec<i32> = count_map
        .iter()
        .map(|x| if *x.0 == 'J' { *x.1 - num_jokers } else { *x.1 })
        .collect();

    let max_counts = counts.iter().max().unwrap();

    if max_counts + num_jokers == 5 {
        return WinType::FiveOfAKind;
    } else if max_counts + num_jokers == 4 {
        return WinType::FourOfAKind;
    } else if counts.iter().any(|x| *x == 3) && counts.iter().any(|x| *x == 2)
        || (counts
            .iter()
            .fold(0, |a, x| if *x == 2 { a + 1 } else { a })
            >= 2
            && num_jokers >= 1)
    {
        return WinType::FullHouse;
    } else if max_counts + num_jokers == 3 {
        return WinType::ThreeOfAKind;
    } else if counts
        .iter()
        .fold(0, |a, x| if *x == 2 { a + 1 } else { a })
        >= 2
    {
        return WinType::TwoPair;
    } else if counts.iter().any(|x| *x == 2) || num_jokers >= 1 {
        return WinType::OnePair;
    } else {
        return WinType::HighCard;
    }
}

fn compare(a: &Hand, b: &Hand) -> Ordering {
    let a_p = a.card_points.clone().unwrap();
    let b_p = b.card_points.clone().unwrap();

    let wintype_a = get_win_type(a);
    let wintype_b = get_win_type(b);

    if (wintype_a as usize) > (wintype_b as usize) {
        return Ordering::Greater;
    } else if (wintype_a as usize) < (wintype_b as usize) {
        return Ordering::Less;
    }

    for i in 0..min(a_p.len(), b_p.len()) {
        if a_p[i] < b_p[i] {
            return Ordering::Less;
        } else if a_p[i] > b_p[i] {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input.txt");

        assert_eq!(process(input), 5905);
    }
}
