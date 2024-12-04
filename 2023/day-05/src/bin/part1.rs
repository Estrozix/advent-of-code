fn main() {
    let input = include_str!("input.txt");

    println!("{}", process(input));
}

#[derive(Debug)]
pub struct Span {
    start_destination: usize,
    start_source: usize,
    length: usize,
}

#[derive(Debug)]
pub struct Category {
    _name: String,
    ranges: Vec<Span>,
}

pub fn process(input: &str) -> usize {
    let map_names = [
        "seeds",
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut seeds: Vec<i64> = Vec::new();
    let mut categories: Vec<Category> = Vec::new();

    let splits: Vec<_> = input
        .split("\n\n")
        .map(|x| x.split_once(":").unwrap().1)
        .collect();

    for (i, split) in splits.iter().enumerate() {
        let ranges: Vec<_> = split
            .split("\n")
            .filter(|x| x.len() > 0)
            .map(|x| String::from(x))
            .collect();

        if i == 0 {
            seeds = ranges[0]
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
        } else {
            let mut category_ranges: Vec<Span> = Vec::new();

            for range in ranges {
                let numbers: Vec<_> = range
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let start_destination = numbers[0];
                let start_source = numbers[1];
                let length = numbers[2];

                category_ranges.push(Span {
                    start_destination,
                    start_source,
                    length,
                });
            }

            categories.push(Category {
                _name: String::from(map_names[i]),
                ranges: category_ranges,
            })
        }
    }

    let mut seed_to_location: Vec<(i64, i64)> = Vec::new();

    for seed in seeds {
        let source = seed;
        
        let location = find_location(source, &categories);

        println!("Seed {} ended up in {}", seed, location);

        seed_to_location.push((seed, location));
    }

    let min_location = seed_to_location.iter().map(|x| x.1).min().unwrap();

    return min_location as usize;
}

pub fn find_location(source: i64, categories: &Vec<Category>) -> i64 {
    let mut source = source;
    for category in categories {
        let mut destination = -1;

        for range in &category.ranges {
            // if in range
            if (source as usize) >= range.start_source && (source as usize) < range.start_source + range.length {
                destination = (source as i64 - range.start_source as i64) + range.start_destination as i64;
            }
        }

        if destination == -1 {
            destination = source;
        }

        source = destination;
    }

    return source;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_process() {
        let input = include_str!("./test_input.txt");

        assert_eq!(process(input), 35);
    }
}
