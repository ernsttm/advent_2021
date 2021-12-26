const _TEST_CONTENTS: &str = "16,1,2,0,4,2,7,1,2,14";

fn parse(contents: &str) -> Vec<i64> {
    contents
        .split(",")
        .map(|p| p.parse::<i64>().unwrap())
        .collect()
}

fn find_minimal_fuel(positions: &[i64]) -> i64 {
    let fuel_consumption =
        |p: i64| -> i64 { positions.iter().fold(0, |acc, pos| acc + (p - pos).abs()) };

    let expensive_fuel_consumption = |p: i64| -> i64 {
        positions.iter().fold(0, |acc, pos| {
            let positional_dif = (p - pos).abs();
            let burn: i64 = positional_dif * (positional_dif + 1) / 2;
            acc + burn
        })
    };

    let median = positions[positions.len() / 2];

    let find_minimum_in_direction = |increment: i64| -> i64 {
        let mut current_minimum = expensive_fuel_consumption(median);
        let mut test_value = median + increment;
        while current_minimum >= expensive_fuel_consumption(test_value) {
            current_minimum = expensive_fuel_consumption(test_value);
            test_value += increment;
        }

        current_minimum
    };

    find_minimum_in_direction(1).min(find_minimum_in_direction(-1))
}

fn main() {
    let contents = std::fs::read_to_string("day7/input.txt").unwrap();
    let mut positions = parse(&contents);
    positions.sort();
    println!(
        "Lowest Consumption position: {}",
        find_minimal_fuel(&positions)
    );
}
