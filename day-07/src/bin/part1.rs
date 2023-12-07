use std::{
    cmp::{min, Ordering},
    collections::HashMap,
};

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

pub fn process(input: &str) -> usize {
    let cards_types = vec![
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

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
                    .map(|c| cards_types.iter().position(|p| *p == c).unwrap() as i32 + 1)
                    .collect(),
            ),
        })
        .collect();

    hands.sort_by(|a, b| compare(a, b));

    let mut answer = 0;

    for (i, hand) in hands.iter().enumerate() {
        answer += hand.bet as usize * (i + 1);
    }

    hands.iter().for_each(|x| println!("{}", x.cards));

    answer
}

fn get_win_type(a: &Hand) -> WinType {
    let counts = a
        .cards
        .chars()
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
            map
        })
        .values()
        .cloned()
        .collect::<Vec<i32>>();

    if counts.iter().any(|x| *x == 5) {
        return WinType::FiveOfAKind;
    } else if counts.iter().any(|x| *x == 4) {
        return WinType::FourOfAKind;
    } else if counts.iter().any(|x| *x == 3) && counts.iter().any(|x| *x == 2) {
        return WinType::FullHouse;
    } else if counts.iter().any(|x| *x == 3) {
        return WinType::ThreeOfAKind;
    } else if counts
        .iter()
        .fold(0, |a, x| if *x == 2 { a + 1 } else { a })
        >= 2
    {
        return WinType::TwoPair;
    } else if counts.iter().any(|x| *x == 2) {
        return WinType::OnePair;
    } else {
        return WinType::HighCard;
    }
}

fn compare(a: &Hand, b: &Hand) -> Ordering {
    let a_p = a.card_points.clone().unwrap();
    let b_p = b.card_points.clone().unwrap();

    if (get_win_type(a) as usize) > (get_win_type(b) as usize) {
        return Ordering::Greater;
    } else if (get_win_type(a) as usize) < (get_win_type(b) as usize) {
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

        assert_eq!(process(input), 6440);
    }
}
