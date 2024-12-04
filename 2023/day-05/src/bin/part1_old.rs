fn main() {
    let input = include_str!("input.txt");

    println!("{}", process(input));
}

#[derive(Debug)]
pub struct Category {
    name: String,
    ranges: Vec<String>,
}

#[derive(Debug)]
pub struct LinkRow {
    name: String,
    links: Vec<Link>,
}

#[derive(Debug)]
pub struct Link {
    source: usize,
    destination: usize,
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

    let mut links: Vec<LinkRow> = Vec::new();

    let mut seeds: Vec<i64> = Vec::new();
    let mut categories: Vec<Category> = Vec::new();

    let splits: Vec<_> = input
    .split("\n\n").map(|x| x.split_once(":").unwrap().1).collect();

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
            categories.push(Category {
                name: String::from(map_names[i]),
                ranges: ranges,
            })
        }
    }

    for (ci, category) in categories.iter().enumerate() {
        for range in &category.ranges {
            let numbers: Vec<i64> = range.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

            let destination_start = numbers[0];
            let source_start = numbers[1];
            let length = numbers[2];

            for i in 0..length {
                if links.get(ci).is_none() {
                    links.push(LinkRow { name: category.name.clone(), links: Vec::<Link>::new() })
                }

                links[ci].links.push(Link {
                    source: (source_start + i) as usize,
                    destination: (destination_start + i) as usize,
                });
            }
        }
    }

    println!("{:?}", links);

    let mut seed_to_location: Vec<Link> = Vec::new();

    for seed in seeds {
        let mut source = seed as usize;

        for row in &links {
            println!("{}", source);

            if let Some(curr_link) = row.links.iter().find(|x| x.source == source) {
                source = curr_link.destination;
            } else {
                source = source;
            }
        }

        seed_to_location.push(Link {
            source: seed as usize,
            destination: source,
        });

        println!("-----");
    }



    let mut min_destination = seed_to_location.first().unwrap().destination;

    for stl in &seed_to_location {
        if stl.destination < min_destination {
            min_destination = stl.destination;
        }
    }

    println!("{:?}", seed_to_location);

    min_destination
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
