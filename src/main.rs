use std::{env, fmt};

#[derive(Clone, Debug)]
enum Cell {
    Alive,
    Dead,
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cell::Alive => 'o',
            Cell::Dead => '.',
        };
        write!(f, "{}", symbol)
    }
}

struct Board {
    w: usize,
    n: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(cells: &str) -> Self {
        let w = (cells.len() as f64).sqrt() as usize;
        let mut cs = Vec::new();
        for c in cells.chars() {
            cs.push(match c {
                '.' => Cell::Dead,
                'o' => Cell::Alive,
                _ => panic!("Unknown cell value. Must be 'o' or '.'."),
            });
        }
        let n = cs.len();
        Self { w, cells: cs, n }
    }

    fn next_board(&mut self) {
        let mut cells = Vec::new();
        for i in 0..self.n {
            cells.push(self.next_cell(i));
        }
        self.cells = cells;
    }

    fn next_cell(&self, i: usize) -> Cell {
        let mut live_count = 0;
        for j in self.neighbours(i) {
            live_count += match self.cells[j] {
                Cell::Alive => 1,
                _ => 0,
            };
        }
        // println!("{} {:#?} {}", i, self.cells[i], live_count);
        match self.cells[i] {
            Cell::Alive => match live_count {
                2 => Cell::Alive,
                3 => Cell::Alive,
                _ => Cell::Dead,
            },
            Cell::Dead => match live_count {
                3 => Cell::Alive,
                _ => Cell::Dead,
            },
        }
    }

    fn neighbours(&self, i: usize) -> Vec<usize> {
        // tl, tc, tr
        // cl, .., cr
        // bl, bc, br
        let is_top_row = i < self.w;
        let is_bottom_row = i >= (self.n - self.w);
        let is_left_col = (i % self.w) == 0;
        let is_right_col = (i >= (self.w - 1)) && (((i + 1) % self.w) == 0);

        // println!(
        //     "{} {} {} {} {}",
        //     i, is_top_row, is_bottom_row, is_left_col, is_right_col
        // );

        match (is_top_row, is_bottom_row, is_left_col, is_right_col) {
            (true, true, true, true) => Vec::new(),
            (true, true, true, false) => vec![i + 1],
            (true, true, false, true) => vec![i - 1],
            (true, false, true, true) => vec![i + self.w],
            (false, true, true, true) => vec![i - self.w],
            (true, true, false, false) => vec![i - 1, i + 1],
            (true, false, false, true) => vec![i - 1, i + self.w - 1, i + self.w],
            (true, false, true, false) => vec![
                // me
                i + 1,
                i + self.w,
                i + self.w + 1,
            ],
            (false, true, true, false) => vec![i - self.w, i - self.w + 1, i + 1],
            (false, true, false, true) => vec![i - self.w - 1, i - self.w, i - self.w + 1, i - 1],
            (false, false, true, true) => vec![i - self.w, i + self.w],

            (true, false, false, false) => {
                vec![i - 1, i + 1, i + self.w - 1, i + self.w, i + self.w + 1]
            }
            (false, true, false, false) => {
                vec![i - self.w - 1, i - self.w, i - self.w + 1, i - 1, i + 1]
            }
            (false, false, true, false) => vec![
                i - self.w,
                i - self.w + 1,
                i + 1,
                i + self.w,
                i + self.w + 1,
            ],
            (false, false, false, true) => vec![
                i - self.w - 1,
                i - self.w,
                i - 1,
                i + self.w - 1,
                i + self.w,
            ],
            (false, false, false, false) => vec![
                i - self.w - 1,
                i - self.w,
                i - self.w + 1,
                i - 1,
                i + 1,
                i + self.w - 1,
                i + self.w,
                i + self.w + 1,
            ],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for (i, cell) in self.cells.iter().enumerate() {
            if (i > 0) && (i % self.w == 0) {
                writeln!(f)?;
            }
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

fn main() {
    println!("Game of Life");

    let args: Vec<String> = env::args().collect();

    let s: &str = &args[1];

    let mut board = Board::new(s);

    for _ in 0..5 {
        println!("{}", board);
        board.next_board();
    }
    println!("{}", board);
}
