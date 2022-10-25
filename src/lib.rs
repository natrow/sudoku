// Copyright 2022 Nathan Rowan
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific
// language governing permissions and limitations under the License.

#[macro_use]
extern crate lazy_static;
use pyo3::prelude::*;

pub mod dancing_links;
pub mod sudoku_alg;

#[pymodule]
fn sudoku(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sudoku_alg::solve, m)?)?;
    m.add_function(wrap_pyfunction!(sudoku_alg::print_puzzle, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::dancing_links::DancingLinks;
    use crate::sudoku_alg::{print_puzzle, solve};

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn dlx_test() -> TestResult {
        // create matrix of boolean values
        let matrix: Vec<bool> = [
            0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1,
        ]
        .iter()
        .map(|x| *x != 0)
        .collect();

        // create DLX representation
        let dlx = DancingLinks::new(&matrix, 7, 6)?;

        println!("Encoded DLX:\n{dlx}");

        let solutions = dlx.solve(None)?;

        for solution in solutions {
            println!("Found solution: {solution:?}");
        }

        Ok(())
    }

    #[test]
    fn sudoku_test() -> TestResult {
        // create sudoku puzzle
        let puzzle = vec![
            4, 0, 6, 7, 3, 5, 8, 1, 0, 2, 7, 8, 0, 9, 6, 5, 4, 0, 0, 0, 0, 2, 0, 0, 7, 9, 0, 0, 6,
            2, 4, 0, 3, 0, 0, 0, 0, 0, 0, 0, 6, 1, 4, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3,
            0, 0, 6, 0, 0, 0, 1, 7, 0, 5, 0, 0, 0, 4, 6, 0, 9, 0, 0, 0, 2, 0, 5,
        ];

        println!("Unsolved puzzle");
        print_puzzle(puzzle.clone())?;

        // solve puzzle
        let solution = solve(puzzle)?;

        println!("Solved puzzle");
        print_puzzle(Vec::from(solution))?;

        Ok(())
    }
}
