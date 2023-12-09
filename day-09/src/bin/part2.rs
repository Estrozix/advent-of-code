fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input));
}

pub fn process(input: &str) -> i32 {
    let mut data: Vec<_> = input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    println!("{data:?}");

    let sum: i32 = data
        .iter_mut()
        .map(|x| {
            x.reverse();
            predict(x)
        })
        .sum();

    sum
}

pub fn predict(seq: &Vec<i32>) -> i32 {
    let mut seqs: Vec<Vec<i32>> = Vec::new();

    seqs.push(seq.clone());

    let mut level = 1;
    let mut done = false;

    while !done {
        let line = seqs[level - 1].clone();

        for i in 0..line.len() - 1 {
            if seqs.get(level).is_none() {
                seqs.push(Vec::<i32>::new());
            }

            seqs[level].push(line[i + 1] - line[i]);
        }

        if seqs[level].iter().all(|x| *x == 0) {
            done = true;
        }

        level += 1;
    }

    let mut inc_by = 0;
    let mut new_value = 0;

    for c_seq in seqs.iter_mut().rev() {
        {
            let curr_value = c_seq.last().unwrap();
            new_value = curr_value + inc_by;
            c_seq.push(new_value);
            inc_by = new_value;
        }
    }

    new_value
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input.txt");

        assert_eq!(process(input), 114);
    }
}
