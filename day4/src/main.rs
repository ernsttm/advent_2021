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

#[derive(Clone, Debug)]
struct BingoBoard {
    grid: [[(usize, bool); 5]; 5],
}

impl BingoBoard {
    fn new(lines: &[&str]) -> BingoBoard {
        let mut grid = [[(0, false); 5]; 5];
        for (i, &line) in lines.iter().enumerate() {
            let numbers = line
                .split(' ')
                .filter(|&l| !l.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            for (j, number) in numbers.iter().enumerate() {
                grid[i][j] = (*number, false);
            }
        }

        BingoBoard { grid }
    }

    fn check_win(&self, (i, j): (usize, usize)) -> bool {
        let horizontal_win = self.grid[i].iter().all(|(_, visited)| *visited);
        let vertical_win = self.grid.iter().all(|row| row[j].1);

        horizontal_win || vertical_win
    }

    fn is_won(&self) -> bool {
        (0..5).any(|i| self.check_win((i, i)))
    }

    fn mark_called_number(&mut self, called: usize) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.grid[i][j].0 == called {
                    self.grid[i][j].1 = true;
                    return self.check_win((i, j));
                }
            }
        }

        false
    }

    fn final_score(&self, last_called: usize) -> usize {
        let unmarked_numbers: usize = self
            .grid
            .iter()
            .map(|row| {
                row.iter().fold(
                    0,
                    |acc, (value, visited)| {
                        if !visited {
                            acc + *value
                        } else {
                            acc
                        }
                    },
                )
            })
            .sum();

        unmarked_numbers * last_called
    }
}

fn find_first_winner(numbers: &[usize], mut boards: Vec<BingoBoard>) -> usize {
    for &number in numbers {
        for board in boards.iter_mut() {
            if board.mark_called_number(number) {
                return board.final_score(number);
            }
        }
    }

    0
}

fn find_last_winner(numbers: &[usize], mut boards: Vec<BingoBoard>) -> usize {
    for &number in numbers {
        let count = boards.len();
        for board in boards.iter_mut() {
            if board.mark_called_number(number) && count == 1 {
                return board.final_score(number);
            }
        }

        boards.retain(|board| !board.is_won());
    }

    0
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
    let contents = std::fs::read_to_string("day4/input.txt").unwrap();
    let (numbers, boards) = parse(&contents);
    println!(
        "First Winner: {}",
        find_first_winner(&numbers, boards.clone())
    );
    println!("Last Winner: {}", find_last_winner(&numbers, boards));
}
