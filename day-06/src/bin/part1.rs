fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

pub fn process(input: &str) -> i32 {
    let data: Vec<_> = input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .skip(1)
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let races: Vec<(i32, i32)> = data[0]
        .clone()
        .into_iter()
        .zip(data[1].clone().into_iter())
        .collect();

    let num_ways_to_win: Vec<i32> = races
        .iter()
        .map(|x| {
            let mut num_ways = 0;
            for t in 0..x.0 {
                let mut total_time = t;

                let speed = t;
                let distance = x.1;

                let time_to_finish = (distance as f32 / speed as f32).floor() as i32;

                total_time += time_to_finish;

                if total_time < x.0 {
                    println!("Holding {} wins in race {:?}!", t, x);
                    num_ways += 1;
                }
            }
            num_ways
        })
        .collect();

    println!("{:?}", num_ways_to_win);

    let product = num_ways_to_win.iter().product();

    product
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input.txt");

        assert_eq!(process(input), 288);
    }
}
