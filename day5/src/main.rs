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
        let dimensions = clouds.iter().fold((0, 0) |acc, cloud| (max(acc.0, cloud)))
    }
}

fn parse(contents: &str) -> OceanFloor {
    let mut clouds = Vec::new();

    let clouds = contents
        .lines()
        .map(|l| {
            let parse_position = |position| -> Position {
                let coordinate_iter = position.splitn(2, ',');
                Position {
                    x: coordinate_iter.next().unwrap(),
                    y: coordinate_iter.next().unwrap(),
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
    println!("Hello, world!");
}
