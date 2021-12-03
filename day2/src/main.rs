use crate::Direction::{Downward, Forward, Upward};
use std::fs;

#[derive(Debug)]
enum Direction {
    Forward(usize),
    Upward(usize),
    Downward(usize),
}

fn parse(contents: &str) -> Vec<Direction> {
    contents
        .lines()
        .map(|l| {
            let l = l.split(' ').collect::<Vec<&str>>();
            let direction = &l[0];
            let magnitude = l[1].parse::<usize>().unwrap();

            if direction == &"forward" {
                Forward(magnitude)
            } else if direction == &"up" {
                Upward(magnitude)
            } else {
                Downward(magnitude)
            }
        })
        .collect::<Vec<Direction>>()
}

fn find_position(directions: &[Direction]) -> usize {
    let position = directions.iter().fold((0, 0), |mut a, d| {
        match d {
            Forward(m) => a.0 += *m,
            Upward(m) => a.1 -= *m,
            Downward(m) => a.1 += *m,
        }

        a
    });

    position.0 * position.1
}

fn find_position_with_aim(directions: &[Direction]) -> usize {
    let position_with_aim = directions.iter().fold((0, 0, 0), |mut a, d| {
        match d {
            Forward(m) => {
                a.0 += m;
                a.1 += m * a.2;
            }
            Upward(m) => a.2 -= m,
            Downward(m) => a.2 += m,
        }

        a
    });

    position_with_aim.0 * position_with_aim.1
}

fn main() {
    const _TEST_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    let contents = fs::read_to_string("day2/input.txt").unwrap();
    let directions = parse(&contents);
    println!("Position {}", find_position(&directions));
    println!("Position with aim {}", find_position_with_aim(&directions));
}
