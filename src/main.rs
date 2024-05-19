use std::{
    env, fmt,
    fs::File,
    io::{self, BufReader, Read},
};

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
    size: usize,
    n: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(initial_state: Vec<Cell>, size: usize, pos: (usize, usize)) -> Self {
        // let mut cs = Vec::new();
        let n = initial_state.len();
        let size = (n as f32).sqrt() as usize;
        Self {
            size,
            cells: initial_state,
            n,
        }
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
        let is_top_row = i < self.size;
        let is_bottom_row = i >= (self.n - self.size);
        let is_left_col = (i % self.size) == 0;
        let is_right_col = (i >= (self.size - 1)) && (((i + 1) % self.size) == 0);

        // println!(
        //     "{} {} {} {} {}",
        //     i, is_top_row, is_bottom_row, is_left_col, is_right_col
        // );

        match (is_top_row, is_bottom_row, is_left_col, is_right_col) {
            (true, true, true, true) => Vec::new(),
            (true, true, true, false) => vec![i + 1],
            (true, true, false, true) => vec![i - 1],
            (true, false, true, true) => vec![i + self.size],
            (false, true, true, true) => vec![i - self.size],
            (true, true, false, false) => vec![i - 1, i + 1],
            (true, false, false, true) => vec![i - 1, i + self.size - 1, i + self.size],
            (true, false, true, false) => vec![
                // me
                i + 1,
                i + self.size,
                i + self.size + 1,
            ],
            (false, true, true, false) => vec![i - self.size, i - self.size + 1, i + 1],
            (false, true, false, true) => {
                vec![i - self.size - 1, i - self.size, i - self.size + 1, i - 1]
            }
            (false, false, true, true) => vec![i - self.size, i + self.size],

            (true, false, false, false) => {
                vec![
                    i - 1,
                    i + 1,
                    i + self.size - 1,
                    i + self.size,
                    i + self.size + 1,
                ]
            }
            (false, true, false, false) => {
                vec![
                    i - self.size - 1,
                    i - self.size,
                    i - self.size + 1,
                    i - 1,
                    i + 1,
                ]
            }
            (false, false, true, false) => vec![
                i - self.size,
                i - self.size + 1,
                i + 1,
                i + self.size,
                i + self.size + 1,
            ],
            (false, false, false, true) => vec![
                i - self.size - 1,
                i - self.size,
                i - 1,
                i + self.size - 1,
                i + self.size,
            ],
            (false, false, false, false) => vec![
                i - self.size - 1,
                i - self.size,
                i - self.size + 1,
                i - 1,
                i + 1,
                i + self.size - 1,
                i + self.size,
                i + self.size + 1,
            ],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for (i, cell) in self.cells.iter().enumerate() {
            if (i > 0) && (i % self.size == 0) {
                writeln!(f)?;
            }
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

fn read_file_contents(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_life_105(path: &str) -> Vec<Cell> {
    let contents = match read_file_contents(path) {
        Ok(contents) => contents,
        Err(_) => panic!("Could not open file {}.", path),
    };
    let mut cells = Vec::new();
    for line in contents.lines() {
        if line.starts_with("#") {
            continue;
        }
        for ch in line.chars() {
            match ch {
                '.' => cells.push(Cell::Dead),
                '*' => cells.push(Cell::Alive),
                _ => panic!("Only '.' and '*' are valid non-comment characters."),
            };
        }
    }
    cells
}

fn main() {
    println!("Game of Life");

    let args: Vec<String> = env::args().collect();

    let path: &str = &args[1];
    let size: &str = &args[2];
    let pos_str: &str = &args[3];
    let mut pos = Vec::new();
    for p in pos_str.split(",") {
        pos.push(p.parse().unwrap())
    }

    println!("{} {} {:#?}", path, size, pos);
    let cells = read_life_105(path);

    let mut board = Board::new(cells, size.parse().unwrap(), (pos[0], pos[1]));

    for _ in 0..5 {
        println!("{}", board);
        board.next_board();
    }
    println!("{}", board);
}
