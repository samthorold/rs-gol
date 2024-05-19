use std::{
    env, fmt,
    fs::File,
    io::{self, BufReader, Read},
    thread::sleep,
    time::Duration,
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
        // size = 10, pos = (3, 3)
        let x = pos.0;
        let y = pos.1;
        let mut cs = Vec::new();
        if y > 0 {
            for _ in 0..y {
                for _ in 0..size {
                    cs.push(Cell::Dead);
                }
            }
        }
        let initial_size = (initial_state.len() as f32).sqrt() as usize;
        for (i, cell) in initial_state.into_iter().enumerate() {
            let is_left_col = (i % initial_size) == 0;
            let is_right_col = (i >= (initial_size - 1)) && (((i + 1) % initial_size) == 0);
            // if new row, add x dead cells
            if is_left_col {
                for _ in 0..x {
                    cs.push(Cell::Dead);
                }
            }
            // add initial cell
            cs.push(cell);
            // if end of row, add (size - x - initial_size) dead cells
            if is_right_col {
                for _ in 0..(size - x - initial_size) {
                    cs.push(Cell::Dead);
                }
            }
        }
        for _ in 0..(size - initial_size) {
            for _ in 0..size {
                cs.push(Cell::Dead);
            }
        }
        Self {
            size,
            cells: cs.clone(),
            n: cs.len(),
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
    let size: usize = args[2].parse().unwrap();
    let pos_str: &str = &args[3];
    let mut pos = Vec::new();
    for p in pos_str.split(",") {
        pos.push(p.parse().unwrap())
    }

    // println!("{} {} {:#?}", path, size, pos);
    let initial_state = read_life_105(path);

    let mut board = Board::new(initial_state, size, (pos[0], pos[1]));

    for _ in 0..25 {
        println!("{}", board);
        sleep(Duration::from_millis(500));
        board.next_board();
    }
    println!("{}", board);
}
