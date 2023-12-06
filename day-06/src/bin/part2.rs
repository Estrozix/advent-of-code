fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

pub fn process(input: &str) -> i64 {
    let data: Vec<_> = input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .skip(1)
                .fold(String::new(), |a, b| a + b)
                .parse::<i64>()
                .unwrap()
        })
        .collect();

    let races = vec![(data[0], data[1])];

    let num_ways_to_win: Vec<i64> = races
        .iter()
        .map(|x| {
            let mut num_ways = 0;
            for t in 0..x.0 {
                let mut total_time = t;

                let speed = t;
                let distance = x.1;

                let time_to_finish = (distance as f64 / speed as f64).floor() as i64;

                total_time += time_to_finish;

                if total_time < x.0 {
                    // println!("Holding {} wins in race {:?}!", t, x);
                    num_ways += 1;
                }
            }
            num_ways
        })
        .collect();


        *num_ways_to_win.first().unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input.txt");

        assert_eq!(process(input), 71503);
    }
}
