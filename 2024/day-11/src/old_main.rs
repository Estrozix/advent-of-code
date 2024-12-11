use std::fs::read_to_string;

#[derive(Clone)]
struct Stone {
    value: String,
    next: Option<Box<Stone>>,
}

impl Stone {
    fn append(&mut self, value: String) {
        match self.next {
            Some(ref mut stone) => {
                stone.append(value);
            }
            None => {
                let stone = Stone { value, next: None };
                self.next = Some(Box::new(stone));
            }
        }
    }

    fn list(&self) {
        print!("{} ", self.value);
        match self.next {
            Some(ref stone) => stone.list(),
            None => println!(""),
        }
    }

    fn count(&self) -> i64 {
        if let Some(ref stone) = self.next {
            return stone.count() + 1;
        } else {
            return 1;
        }
    }

    fn blink(&mut self) {
        let length = self.value.chars().count();

        match &mut self.next {
            Some(stone) => stone.blink(),
            None => {}
        }

        if self.value == "0" {
            self.value = "1".to_string();
        } else if length % 2 == 0 {
            let parts = self.value.split_at(length / 2);

            let next_stone: Option<Box<Stone>>;

            if let Some(stone) = &self.next {
                next_stone = Some(stone.clone());
            } else {
                next_stone = None;
            }

            let new_stone = Stone {
                value: parts
                    .1
                    .parse::<u64>()
                    .expect("Should be able to parse")
                    .to_string(),
                next: next_stone,
            };

            self.value = parts
                .0
                .parse::<u64>()
                .expect("Should be able to parse")
                .to_string();
            self.next = Some(Box::new(new_stone));
        } else {
            self.value =
                (self.value.parse::<u64>().expect("All should be parsable") * 2024).to_string()
        }
    }
}

fn main() {
    let input = read_to_string("test.txt").unwrap();

    let initial_stones: Vec<String> = input
        .split(" ")
        .map(|stone| String::from(stone.replace("\n", "")))
        .collect();

    let mut first_stone = Stone {
        next: None,
        value: initial_stones.first().unwrap().clone(),
    };

    for stone in initial_stones.iter().skip(1) {
        first_stone.append(stone.clone());
    }

    for _ in 0..6 {
        first_stone.blink();
        first_stone.list();
    }

    let num = first_stone.count();

    println!("Total num: {:?}", num);
}
