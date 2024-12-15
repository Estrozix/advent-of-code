use std::fs::read_to_string;

#[derive(Debug)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let parts: Vec<&str> = input.split("\n\n").collect();

    let machines: Vec<Machine> = parts
        .iter()
        .map(|part| {
            let lines: Vec<&str> = part.lines().collect();

            let button_a = get_button_data(lines.first().expect("Should be there"));
            let button_b = get_button_data(lines.get(1).expect("Should be there"));
            let price = get_price_data(lines.get(2).expect("Should be there"));

            return Machine {
                ax: button_a.0,
                ay: button_a.1,
                bx: button_b.0,
                by: button_b.1,
                px: price.0 + 10000000000000,
                py: price.1 + 10000000000000,
            };
        })
        .collect();

    println!("{:?}", machines);

    let result = machines.iter().fold(0, |acc, el| {
        if let Some(solution) = solve_machine(el) {
            return acc + 3 * solution.0 + 1 * solution.1;
        } else {
            return acc;
        }
    });
    println!("{:?}", result);
}

fn solve_machine(machine: &Machine) -> Option<(i64, i64)> {
    let Machine {
        ax,
        ay,
        bx,
        by,
        px,
        py,
    } = machine;

    let num_a = (by * px - bx * py) as f64 / (ax * by - bx * ay) as f64;
    let num_b_top = *py as f64 - (*ay as f64 * num_a);
    let num_b_bottom = by;

    let num_b = num_b_top as f64 / *num_b_bottom as f64;

    if num_a.fract() == 0.0 && num_b.fract() == 0.0 {
        println!("Works!");
        println!("a = {}, b = {}", num_a, num_b);
        return Some((num_a as i64, num_b as i64));
    } else {
        println!("Doesn't work!");
        return None;
    }
}

fn get_price_data(line: &str) -> (i64, i64) {
    let price_raw = line
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_once(",")
        .unwrap();

    let price = (
        price_raw
            .0
            .trim()
            .replace("X", "")
            .replace("Y", "")
            .replace("=", "")
            .parse::<i64>()
            .expect("Should parse"),
        price_raw
            .1
            .trim()
            .replace("X", "")
            .replace("Y", "")
            .replace("=", "")
            .parse::<i64>()
            .expect("Should parse"),
    );

    return price;
}

fn get_button_data(line: &str) -> (i64, i64) {
    let button_a_raw = line
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_once(",")
        .unwrap();

    let button_a = (
        button_a_raw
            .0
            .trim()
            .replace("X", "")
            .replace("Y", "")
            .replace("+", "")
            .parse::<i64>()
            .expect("Should parse"),
        button_a_raw
            .1
            .trim()
            .replace("X", "")
            .replace("Y", "")
            .replace("+", "")
            .parse::<i64>()
            .expect("Should parse"),
    );

    return button_a;
}
