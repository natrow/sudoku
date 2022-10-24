use crate::dancing_links::*;

// Row-Column | Row-Number | Column-Number | Box-Number
const WIDTH: usize = 9 * 9 * 4;

// Row | Column | Number
const HEIGHT: usize = 9 * 9 * 9;

// create matrix to be used for sudoku puzzles
lazy_static! {
    static ref MATRIX: Vec<bool> = create_matrix();
}

#[derive(Debug)]
pub enum SudokuError {
    InvalidCell { got: u8 },
    DancingLinks { inner: DancingLinksError },
    MultipleSolutions { found: usize },
}

impl std::fmt::Display for SudokuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuError::InvalidCell { got } => {
                write!(f, "invalid cell: got {got}!")
            }
            SudokuError::MultipleSolutions { found } => {
                write!(f, "multiple solutions found: {found} solutions!")
            }
            SudokuError::DancingLinks { inner } => {
                write!(f, "dancing links error: {inner}")
            }
        }
    }
}

impl std::error::Error for SudokuError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SudokuError::DancingLinks { inner } => Some(inner),
            _ => None,
        }
    }
}

impl From<DancingLinksError> for SudokuError {
    fn from(error: DancingLinksError) -> Self {
        SudokuError::DancingLinks { inner: error }
    }
}

fn create_matrix() -> Vec<bool> {
    let mut matrix = Vec::with_capacity(HEIGHT * WIDTH);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            // R#
            let r = i / 81;
            // C#
            let c = (i / 9) % 9;
            // N#
            let n = i % 9;
            // B#
            let b = (r / 3) * 3 + (c / 3);

            matrix.push(
                j == (r * 9 + c)
                    || j == (r * 9 + n + 81)
                    || j == (c * 9 + n + 81 * 2)
                    || j == (b * 9 + n + 81 * 3),
            );
        }
    }

    matrix
}

fn decode_solution(solution: &[usize]) -> [u8; 81] {
    let mut puzzle = [0; 81];

    for i in solution {
        let r = i / 81;
        let c = (i / 9) % 9;
        let n = i % 9;

        puzzle[r * 9 + c] = (n + 1) as u8;
    }

    puzzle
}

fn encode_puzzle(puzzle: &[u8; 81]) -> Result<Vec<usize>, SudokuError> {
    let mut partial_solution = Vec::new();

    for r in 0..9 {
        for c in 0..9 {
            let n = puzzle[r * 9 + c];

            match n {
                0 => {}
                1..=9 => {
                    partial_solution.push(r * 81 + c * 9 + (n - 1) as usize);
                }
                x => return Err(SudokuError::InvalidCell { got: x }),
            }
        }
    }

    Ok(partial_solution)
}

pub fn solve(puzzle: &[u8; 81]) -> Result<[u8; 81], SudokuError> {
    let dlx = DancingLinks::new(&MATRIX[..], WIDTH, HEIGHT)?;

    let partial_solution = encode_puzzle(puzzle)?;

    let solutions = dlx.solve(Some(&partial_solution[..]))?;

    match solutions.len() {
        1 => Ok(decode_solution(&solutions[0][..])),
        n => Err(SudokuError::MultipleSolutions { found: n }),
    }
}

pub fn print_puzzle(puzzle: &[u8; 81]) {
    for row in 0..9 {
        for col in 0..9 {
            match puzzle[row * 9 + col] {
                0 => print!(" "),
                n => print!("{n}"),
            }
            if col != 8 {
                print!(" ");
                if col % 3 == 2 {
                    print!("| ");
                }
            } else {
                println!();
            }
        }
        if row % 3 == 2 && row != 8 {
            println!("------+-------+------");
        }
    }
}
