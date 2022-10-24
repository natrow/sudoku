#[macro_use]
extern crate lazy_static;

pub mod dancing_links;
pub mod sudoku;

#[cfg(test)]
mod tests {
    use crate::dancing_links::DancingLinks;
    use crate::sudoku::{print_puzzle, solve};

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
        let puzzle = [
            4, 0, 6, 7, 3, 5, 8, 1, 0, 2, 7, 8, 0, 9, 6, 5, 4, 0, 0, 0, 0, 2, 0, 0, 7, 9, 0, 0, 6,
            2, 4, 0, 3, 0, 0, 0, 0, 0, 0, 0, 6, 1, 4, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3,
            0, 0, 6, 0, 0, 0, 1, 7, 0, 5, 0, 0, 0, 4, 6, 0, 9, 0, 0, 0, 2, 0, 5,
        ];

        println!("Unsolved puzzle");
        print_puzzle(&puzzle);

        // solve puzzle
        let solution = solve(&puzzle)?;

        println!("Solved puzzle");
        print_puzzle(&solution);

        Ok(())
    }
}
