use std::cmp::max;

const _TEST_CONTENTS: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct Cloud {
    start: Position,
    end: Position,
}

struct OceanFloor {
    clouds: Vec<Vec<usize>>,
}

impl OceanFloor {
    fn new(clouds: &[Cloud]) -> OceanFloor {
        let dimensions = clouds.iter().fold((0, 0), |acc, cloud| {
            let max_x = max(cloud.start.x, cloud.end.x);
            let max_y = max(cloud.start.y, cloud.end.y);

            (max(acc.0, max_x + 1), max(acc.1, max_y + 1))
        });

        let mut grid = Vec::new();
        for _ in 0..dimensions.0 {
            let mut row = Vec::new();
            row.resize(dimensions.1, 0);
            grid.push(row);
        }

        for cloud in clouds.iter() {
            if cloud.start.x == cloud.end.x || cloud.start.y == cloud.end.y {
                let x_range = if cloud.start.x < cloud.end.x {
                    cloud.start.x..=cloud.end.x
                } else {
                    cloud.end.x..=cloud.start.x
                };

                let y_range = if cloud.start.y < cloud.end.y {
                    cloud.start.y..=cloud.end.y
                } else {
                    cloud.end.y..=cloud.start.y
                };

                for x in x_range.clone() {
                    for y in y_range.clone() {
                        grid[x][y] += 1;
                    }
                }
            } else {
                let x_range = if cloud.start.x < cloud.end.x {
                    (cloud.start.x..=cloud.end.x).collect::<Vec<usize>>()
                } else {
                    (cloud.end.x..=cloud.start.x).rev().collect::<Vec<usize>>()
                };

                let y_range = if cloud.start.y < cloud.end.y {
                    (cloud.start.y..=cloud.end.y).collect::<Vec<usize>>()
                } else {
                    (cloud.end.y..=cloud.start.y).rev().collect::<Vec<usize>>()
                };

                for (x, y) in x_range.into_iter().zip(y_range) {
                    grid[x][y] += 1
                }
            }
        }

        OceanFloor { clouds: grid }
    }

    fn number_of_unsafe_coordinates(&self) -> usize {
        self.clouds
            .iter()
            .map(|row| row.iter().filter(|&c| *c >= 2).count())
            .sum()
    }
}

fn parse(contents: &str) -> OceanFloor {
    let clouds = contents
        .lines()
        .map(|l| {
            let parse_position = |position: &str| -> Position {
                let mut coordinate_iter = position.splitn(2, ',');
                Position {
                    x: coordinate_iter.next().unwrap().parse::<usize>().unwrap(),
                    y: coordinate_iter.next().unwrap().parse::<usize>().unwrap(),
                }
            };

            let points = l.splitn(2, " -> ").collect::<Vec<&str>>();
            let start = parse_position(points[0]);
            let end = parse_position(points[1]);

            Cloud { start, end }
        })
        .collect::<Vec<Cloud>>();

    OceanFloor::new(&clouds)
}

fn main() {
    let contents = std::fs::read_to_string("day5/input.txt").unwrap();
    let ocean_floor = parse(&contents);
    println!(
        "Unsafe coordinates: {}",
        ocean_floor.number_of_unsafe_coordinates()
    );
}
