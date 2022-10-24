pub mod dancing_links;

#[cfg(test)]
mod tests {
    use crate::dancing_links::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn dlx_test() -> TestResult {
        // create matrix of boolean values
        let matrix = [
            0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1,
        ]
        .iter()
        .map(|x| *x != 0)
        .collect();

        // create DLX representation
        let dlx = DancingLinks::new(&matrix, 7, 6)?;

        let mut dlx_clone = dlx.clone();

        println!("Output graph:\n{dlx}");

        // hide column A
        dlx_clone.cover(1);

        println!("Graph after hiding first column:\n{dlx_clone}");

        // unhide column A
        dlx_clone.uncover(1);

        assert_eq!(dlx, dlx_clone);

        Ok(())
    }
}
