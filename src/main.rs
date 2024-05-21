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
            Cell::Alive => '0',
            Cell::Dead => ' ',
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
    fn new(
        initial_state: Vec<Cell>,
        initial_size: (usize, usize),
        end_size: (usize, usize),
        end_pos: (usize, usize),
    ) -> Self {
        let initial_width = initial_size.0;
        let initial_height = initial_size.1;
        let end_width = end_size.0;
        let end_height = end_size.1;
        let x = end_pos.0;
        let y = end_pos.1;
        let mut cs = Vec::new();
        if y > 0 {
            for _ in 0..y {
                for _ in 0..end_width {
                    cs.push(Cell::Dead);
                }
            }
        }
        for (i, cell) in initial_state.into_iter().enumerate() {
            let is_left_col = (i % initial_width) == 0;
            let is_right_col = (i >= (initial_width - 1)) && (((i + 1) % initial_width) == 0);
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
                for _ in 0..(end_width - x - initial_width) {
                    cs.push(Cell::Dead);
                }
            }
        }
        for _ in 0..(end_height - y - initial_height) {
            for _ in 0..end_width {
                cs.push(Cell::Dead);
            }
        }
        Self {
            size: end_width,
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

fn read_plaintext(path: &str) -> (Vec<Cell>, (usize, usize)) {
    let contents = match read_file_contents(path) {
        Ok(contents) => contents,
        Err(_) => panic!("Could not open file {}.", path),
    };
    let mut cells = Vec::new();
    let mut height: usize = 0;
    let mut width: usize = 0;
    for l in contents.lines() {
        let line = l.trim();
        if line.starts_with("#") || line.starts_with("!") {
            continue;
        }
        height += 1;
        width = line.len();
        for ch in line.chars() {
            match ch {
                '.' => cells.push(Cell::Dead),
                '*' => cells.push(Cell::Alive),
                'o' => cells.push(Cell::Alive),
                'O' => cells.push(Cell::Alive),
                _ => panic!("Only '.', '*', and 'o' are valid non-comment characters."),
            };
        }
    }
    (cells, (width, height))
}

fn read_rle(path: &str) -> (Vec<Cell>, (usize, usize)) {
    let contents = match read_file_contents(path) {
        Ok(contents) => contents,
        Err(_) => panic!("Could not open file {}.", path),
    };
    let mut cells = Vec::new();
    let mut height: usize = 0;
    let mut width: usize = 0;
    let mut have_read_size = false;
    for l in contents.lines() {
        let line = l.trim();
        if line.starts_with("#") {
            continue;
        }
        // assume all files have the x = ..., y = ..., ... line
        if !have_read_size {
            for meta in l.split(",") {
                println!("{}", meta);
                match meta.trim().chars().nth(0).unwrap() {
                    'x' => {
                        let value_str: Vec<&str> = meta.split("=").collect();
                        width = value_str[1].trim().parse().unwrap();
                        println!("{}", width);
                    }
                    'y' => {
                        let value_str: Vec<&str> = meta.split("=").collect();
                        height = value_str[1].trim().parse().unwrap();
                        println!("{}", height);
                    }
                    _ => {}
                }
            }
            have_read_size = true;
        } else {
            if (width == 0) || (height == 0) {
                panic!(
                    "Something has gone wrong. Width = {}, height = {}",
                    width, height
                )
            }
            let mut count_str = String::new();
            let mut line_width: usize = 0;
            for ch in line.chars() {
                match ch {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => count_str.push(ch),
                    '$' => {
                        println!("New line {} {}", width, line_width);
                        for _ in 0..(width - line_width) {
                            cells.push(Cell::Dead);
                        }
                        line_width = 0;
                        // count_str = String::new();
                    }
                    'b' | 'o' => {
                        let cell_value = match ch {
                            'b' => Cell::Dead,
                            'o' => Cell::Alive,
                            _ => panic!("Unclear how we got here."),
                        };
                        if count_str.is_empty() {
                            count_str = String::from("1")
                        }
                        let count: usize = count_str.parse().unwrap();
                        for _ in 0..count {
                            cells.push(cell_value.clone());
                        }
                        count_str = String::new();
                        line_width += count;
                        println!("Cell run {:#?} {} {}", cell_value, count, line_width);
                    }
                    '!' => {}
                    _ => panic!("Unknown symbol {}", ch),
                }
            }
        }
    }
    (cells, (width, height))
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
    let board_info = if path.ends_with(".rle") {
        read_rle(path)
    } else {
        read_plaintext(path)
    };
    let initial_state = board_info.0;
    let initial_size = board_info.1;

    let mut board = Board::new(initial_state, initial_size, (size, size), (pos[0], pos[1]));

    for _ in 0..1_000 {
        println!("{}", board);
        sleep(Duration::from_millis(100));
        board.next_board();
    }
    println!("{}", board);
}
