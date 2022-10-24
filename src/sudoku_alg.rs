use crate::dancing_links::{DancingLinks, Error as DlxError};
use pyo3::{exceptions::PyTypeError, prelude::*};

// Row-Column | Row-Number | Column-Number | Box-Number
const WIDTH: usize = 9 * 9 * 4;

// Row | Column | Number
const HEIGHT: usize = 9 * 9 * 9;

// create matrix to be used for sudoku puzzles
lazy_static! {
    static ref MATRIX: Vec<bool> = create_matrix();
}

#[derive(Debug)]
pub enum Error {
    InvalidGrid { got: usize },
    InvalidCell { got: usize },
    DancingLinks { inner: DlxError },
    MultipleSolutions { found: usize },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidGrid { got } => {
                write!(f, "invalid grid length: got {got}, expected 81!")
            }
            Error::InvalidCell { got } => {
                write!(f, "invalid cell: got {got}!")
            }
            Error::MultipleSolutions { found } => {
                write!(f, "multiple solutions found: {found} solutions!")
            }
            Error::DancingLinks { inner } => {
                write!(f, "dancing links error: {inner}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DancingLinks { inner } => Some(inner),
            _ => None,
        }
    }
}

impl From<DlxError> for Error {
    fn from(error: DlxError) -> Self {
        Error::DancingLinks { inner: error }
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        PyErr::new::<PyTypeError, _>(err.to_string())
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

fn decode_solution(solution: &[usize]) -> [usize; 81] {
    let mut puzzle = [0; 81];

    for i in solution {
        let r = i / 81;
        let c = (i / 9) % 9;
        let n = i % 9;

        puzzle[r * 9 + c] = n + 1;
    }

    puzzle
}

fn encode_puzzle(puzzle: &[usize]) -> Result<Vec<usize>, Error> {
    let mut partial_solution = Vec::new();

    for r in 0..9 {
        for c in 0..9 {
            let n = puzzle[r * 9 + c];

            match n {
                0 => {}
                1..=9 => {
                    partial_solution.push(r * 81 + c * 9 + n - 1);
                }
                x => return Err(Error::InvalidCell { got: x }),
            }
        }
    }

    Ok(partial_solution)
}

#[pyfunction]
pub fn solve(puzzle: Vec<usize>) -> Result<[usize; 81], Error> {
    if puzzle.len() != 81 {
        return Err(Error::InvalidGrid { got: puzzle.len() });
    }

    let dlx = DancingLinks::new(&MATRIX[..], WIDTH, HEIGHT)?;

    let partial_solution = encode_puzzle(&puzzle)?;

    let solutions = dlx.solve(Some(&partial_solution[..]))?;

    match solutions.len() {
        1 => Ok(decode_solution(&solutions[0][..])),
        n => Err(Error::MultipleSolutions { found: n }),
    }
}

#[pyfunction]
pub fn print_puzzle(puzzle: Vec<usize>) -> Result<(), Error> {
    if puzzle.len() != 81 {
        return Err(Error::InvalidGrid { got: puzzle.len() });
    }

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

    Ok(())
}
