use std::collections::HashSet;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> (Vec<usize>, Vec<Bingo>) {
    let mut inputs = input.split("\n\n");
    let mut numbers: Vec<usize> = Vec::new();
    let mut bingos: Vec<Bingo> = Vec::new();
    // Parse number list on first line
    for num_str in inputs.next().unwrap().split(",") {
        let num = num_str
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("Unable to parse {:?} as usize", num_str));
        numbers.push(num);
    }
    // Build Bingo matrixes
    for bingo_str in inputs {
        let mut board = [[Tile::Unmarked(0); 5]; 5];
        let mut uniques: HashSet<usize> = HashSet::new();
        for (i, line) in bingo_str.lines().enumerate() {
            for (j, num_str) in line.split_ascii_whitespace().enumerate() {
                let num = num_str
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Unable to parse {:?} as usize", num_str));
                uniques.insert(num);
                board[i][j] = Tile::Unmarked(num);
            }
        }
        bingos.push(Bingo { board });
    }
    (numbers, bingos)
}

#[aoc(day4, part1)]
pub fn part1((numbers, boards): &(Vec<usize>, Vec<Bingo>)) -> Result<usize, &str> {
    let mut boards = boards.clone();
    for number in numbers {
        boards = boards
            .iter()
            .map(|board| -> Bingo {
                let b = board.mark(*number);
                b
            })
            .collect::<Vec<Bingo>>();
        for board in boards.iter() {
            if board.win() {
                return Ok(board.unmarked_sum() * number);
            }
        }
    }
    Err("Did not find winner")
}

#[aoc(day4, part2)]
pub fn part2((numbers, boards): &(Vec<usize>, Vec<Bingo>)) -> Result<usize, &str> {
    let mut boards = boards.clone();
    let num_boards = boards.len();
    let mut num_wins = 0;
    let mut last_num = 0;
    for number in numbers {
        if num_wins == num_boards {
            break;
        }
        last_num = *number;
        boards = boards
            .iter()
            .map(|board| -> Bingo {
                let b = board.mark(*number);
                b
            })
            .filter(|board| {
                let win = board.win();
                if win {
                    num_wins += 1;
                }
                !win || num_wins == num_boards
            })
            .collect::<Vec<Bingo>>();
    }
    println!("Remaining boards: {:?}", boards);
    match boards.first() {
        Some(board) => Ok(board.unmarked_sum() * last_num),
        None => Err("Did not find winner"),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bingo {
    board: [[Tile; 5]; 5],
}

impl Bingo {
    fn mark(self, num: usize) -> Self {
        let board = self.board.map(|row| row.map(|tile| tile.mark(num)));
        Bingo { board }
    }

    fn unmarked_sum(self) -> usize {
        let mut acc = 0;
        for row in self.board {
            for tile in row {
                match tile {
                    Tile::Unmarked(n) => acc += n,
                    Tile::Marked(_) => (),
                }
            }
        }
        acc
    }

    fn win(self) -> bool {
        // Check rows
        for row in self.board {
            if row.iter().all(|t| t.marked()) {
                return true;
            }
        }
        // Check columns
        for column in 0..5 {
            let mut win = true;
            for i in 0..5 {
                win = win && self.board[i][column].marked();
            }
            if win {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Unmarked(usize),
    Marked(usize),
}

impl Tile {
    fn mark(self, num: usize) -> Self {
        match self {
            Self::Unmarked(n) => {
                if n == num {
                    Self::Marked(n)
                } else {
                    Self::Unmarked(n)
                }
            }
            Self::Marked(n) => Self::Marked(n),
        }
    }

    fn marked(self) -> bool {
        match self {
            Self::Unmarked(_) => false,
            Self::Marked(_) => true,
        }
    }
}
