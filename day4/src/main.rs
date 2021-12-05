const _TEST_CONTENT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

#[derive(Debug)]
struct BingoBoard {
    grid: [[(usize, bool); 5]; 5],
}

impl BingoBoard {
    fn new(lines: &[&str]) -> BingoBoard {
        let mut grid = [[(0, false); 5]; 5];
        for (i, &line) in lines.iter().enumerate() {
            let numbers = line.split(' ').filter(|&l| !l.is_empty()
            )
            println!("Line {}", line);
            for (j, number) in line.split(' ').enumerate() {
                grid[i][j] = (number.parse::<usize>().unwrap(), false)
            }
        }

        BingoBoard { grid }
    }
}

fn parse(contents: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let mut numbers = Vec::new();
    let mut boards = Vec::new();
    let mut board_lines = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if i == 0 {
            numbers = line
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        } else if !line.is_empty() {
            board_lines.push(line);

            if board_lines.len() == 5 {
                boards.push(BingoBoard::new(&board_lines));
                board_lines.clear()
            }
        }
    }

    (numbers, boards)
}

fn main() {
    let (numbers, boards) = parse(_TEST_CONTENT);
    println!("Numbers: {:?}", numbers);
    println!("Boards: {:?}", boards);
}
