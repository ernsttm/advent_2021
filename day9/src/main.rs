use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

const _TEST_CONTENTS: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

struct CaveMap {
    map: Vec<Vec<u8>>,
}

impl CaveMap {
    fn low_points(&self) -> Vec<(usize, usize)> {
        let x_range = 0 as i64..self.map.len() as i64;
        let y_range = 0 as i64..self.map[0].len() as i64;

        let is_point_lower = |x: i64, y: i64, value| -> bool {
            if x_range.contains(&x) && y_range.contains(&y) {
                value < self.map[x as usize][y as usize]
            } else {
                true
            }
        };

        let mut low_points = Vec::new();
        for x in x_range.clone() {
            for y in y_range.clone() {
                let value = self.map[x as usize][y as usize];
                let mut is_low_point = true;
                is_low_point &= is_point_lower(x - 1, y, value);
                is_low_point &= is_point_lower(x + 1, y, value);
                is_low_point &= is_point_lower(x, y + 1, value);
                is_low_point &= is_point_lower(x, y - 1, value);

                if is_low_point {
                    low_points.push((x as usize, y as usize));
                }
            }
        }

        low_points
    }

    fn risk_level(&self) -> usize {
        self.low_points()
            .iter()
            .map(|&(x, y)| self.map[x][y] as usize + 1)
            .sum()
    }

    fn basin_sizes(&self) -> usize {
        let x_range = 0 as i64..self.map.len() as i64;
        let y_range = 0 as i64..self.map[0].len() as i64;

        let mut basin_sizes = Vec::new();
        let low_points = self.low_points();
        for low_point in low_points.iter() {
            let mut traversed = HashSet::new();
            let mut stack = Vec::new();
            stack.push((low_point.0 as i64, low_point.1 as i64));

            let mut is_in_basin = |x: i64, y: i64, stack: &mut Vec<(i64, i64)>| {
                if x_range.contains(&x) && y_range.contains(&y) {
                    if !traversed.contains(&(x, y)) && self.map[x as usize][y as usize] != 9 {
                        traversed.insert((x, y));
                        stack.push((x, y));
                    }
                }
            };

            while !stack.is_empty() {
                let (x, y) = stack.pop().unwrap();
                is_in_basin(x - 1, y, &mut stack);
                is_in_basin(x + 1, y, &mut stack);
                is_in_basin(x, y - 1, &mut stack);
                is_in_basin(x, y + 1, &mut stack);
            }

            basin_sizes.push(traversed.len());
        }

        basin_sizes.sort();
        basin_sizes.reverse();
        basin_sizes[0..3].iter().fold(1, |acc, size| acc * size)
    }
}

impl Debug for CaveMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            writeln!(
                f,
                "{}",
                row.iter().map(|u| u.to_string()).collect::<String>()
            )?;
        }

        Ok(())
    }
}

fn parse(contents: &str) -> CaveMap {
    let map = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    CaveMap { map }
}

fn main() {
    let contents = std::fs::read_to_string("day9/input.txt").unwrap();
    let cave_map = parse(&contents);
    println!("Risk Level: {:?}", cave_map.risk_level());
    println!("Basin sizeL {:?}", cave_map.basin_sizes());
}
