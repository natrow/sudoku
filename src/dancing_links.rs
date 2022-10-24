#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Node {
    l: usize,         // left
    r: usize,         // right
    u: usize,         // up
    d: usize,         // down
    c: usize,         // column
    p: Option<Point>, // node position
}

impl Node {
    fn new(id: usize, p: Option<Point>) -> Self {
        Self {
            l: id,
            r: id,
            u: id,
            d: id,
            c: id,
            p,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(point) = &self.p {
            write!(
                f,
                "L[{}] R[{}] U[{}] D[{}] C[{}] ({}, {})",
                self.l, self.r, self.u, self.d, self.c, point.x, point.y
            )
        } else {
            write!(
                f,
                "L[{}] R[{}] U[{}] D[{}] C[{}] (N/A)",
                self.l, self.r, self.u, self.d, self.c
            )
        }
    }
}

#[derive(Debug, Clone)]
pub enum DancingLinksError {
    InvalidMatrixSize { expected: usize, got: usize },
    InvalidPartialSolution { row: usize },
    InternalError { msg: String },
    NoSolutions,
}

impl std::fmt::Display for DancingLinksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DancingLinksError::InvalidMatrixSize { expected, got } => {
                write!(
                    f,
                    "invalid matrix dimensions, expected {expected} elements, got {got}!"
                )
            }
            DancingLinksError::InvalidPartialSolution { row } => {
                write!(
                    f,
                    "invalid partial solution entered: row {row} is part of header!"
                )
            }
            DancingLinksError::InternalError { msg } => {
                write!(f, "internal error occurred: {msg}!")
            }
            DancingLinksError::NoSolutions => {
                write!(f, "no solutions found!")
            }
        }
    }
}

impl std::error::Error for DancingLinksError {}

#[derive(Debug, Clone, PartialEq)]
pub struct DancingLinks {
    grid: Vec<Node>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for DancingLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write as _;

        let mut out = String::new();

        let grid = &self.grid;

        let mut i = 0;
        loop {
            let mut j = i;
            loop {
                writeln!(out, "Node {}: {}", j, grid[j]).unwrap();

                j = grid[j].d;

                if i == j {
                    break;
                }
            }

            i = grid[i].r;

            if i == 0 {
                break;
            }
        }

        write!(f, "{}", out)
    }
}

impl DancingLinks {
    pub fn new(matrix: &[bool], width: usize, height: usize) -> Result<Self, DancingLinksError> {
        // check that dimensions are valid
        if matrix.len() == width * height {
            // count number of nodes
            let num_ones: usize = matrix.iter().map(|b| if *b { 1 } else { 0 }).sum();

            // root + columns + nodes
            let mut grid = Vec::with_capacity(1 + width + num_ones);

            // create root node
            let root_id = grid.len();
            grid.push(Node::new(root_id, None));

            // create column nodes
            let mut rcol_id = root_id;
            for i in 0..width {
                let col_id = grid.len();
                grid.push(Node::new(col_id, None));

                // knit into row
                grid[root_id].l = col_id;
                grid[col_id].r = root_id;
                grid[rcol_id].r = col_id;
                grid[col_id].l = rcol_id;
                rcol_id = col_id;

                // create column
                let mut trow_id = col_id;
                for j in 0..height {
                    // check if node should be created
                    if matrix[j * width + i] {
                        // create node at coordinate point
                        let row_id = grid.len();
                        grid.push(Node::new(row_id, Some(Point { x: i, y: j })));

                        // attach column id
                        grid[row_id].c = col_id;

                        // knit into column
                        grid[col_id].u = row_id;
                        grid[row_id].d = col_id;
                        grid[trow_id].d = row_id;
                        grid[row_id].u = trow_id;
                        trow_id = row_id;

                        // knit into start of row
                        let mut row_start: Option<usize> = None;
                        for (cell_id, cell) in grid.iter().enumerate() {
                            if let Some(point) = &cell.p {
                                if point.y == j && point.x != i {
                                    row_start = Some(cell_id);
                                    break;
                                }
                            }
                        }
                        if let Some(row_start) = row_start {
                            grid[row_start].l = row_id;
                            grid[row_id].r = row_start;

                            // knit into end of row
                            let mut row_end: Option<usize> = None;
                            for (cell_id, cell) in grid.iter().enumerate().rev() {
                                if let Some(point) = &cell.p {
                                    if point.y == j && point.x != i && cell_id >= row_start {
                                        row_end = Some(cell_id);
                                        break;
                                    }
                                }
                            }
                            if let Some(row_end) = row_end {
                                grid[row_end].r = row_id;
                                grid[row_id].l = row_end;
                            } else {
                                return Err(DancingLinksError::InternalError {
                                    msg: "couldn't find end of row".to_owned(),
                                });
                            }
                        }
                    }
                }
            }

            if grid.len() == 1 + width + num_ones {
                let dlx = DancingLinks {
                    grid,
                    width,
                    height,
                };

                Ok(dlx)
            } else {
                Err(DancingLinksError::InternalError {
                    msg: "incorrect grid length".to_owned(),
                })
            }
        } else {
            Err(DancingLinksError::InvalidMatrixSize {
                expected: width * height,
                got: matrix.len(),
            })
        }
    }

    fn cover(&mut self, c: usize) {
        let grid = &mut self.grid;

        // Step 1: Hide column header
        // L[R[c]] <- L[c]
        // R[L[c]] <- R[c]

        let r_c = grid[c].r;
        let l_c = grid[c].l;
        grid[r_c].l = l_c;
        grid[l_c].r = r_c;

        // Step 2: Iterate through rows in column (downwards)
        let mut i = grid[c].d;
        while i != c {
            // Step 3: Iterate through cells in row (rightwards)
            let mut j = grid[i].r;
            while j != i {
                // Step 4: Hide cells
                // U[D[j]] <- U[j]
                // D[U[j]] <- D[j]

                let d_j = grid[j].d;
                let u_j = grid[j].u;
                grid[d_j].u = u_j;
                grid[u_j].d = d_j;

                j = grid[j].r;
            }

            i = grid[i].d
        }
    }

    fn uncover(&mut self, c: usize) {
        let grid = &mut self.grid;

        // Step 1: Iterate through rows in column (upwards)
        let mut i = grid[c].u;
        while i != c {
            // Step 2: Iterate through cells in row (leftwards)
            let mut j = grid[i].l;
            while j != i {
                // Step 3: Unhide cells
                // U[D[j]] <- j
                // D[U[j]] <- j

                let d_j = grid[j].d;
                let u_j = grid[j].u;
                grid[d_j].u = j;
                grid[u_j].d = j;

                j = grid[j].l;
            }

            i = grid[i].u;
        }
        // Step 4: Unhide column
        // L[R[c]] <- c
        // R[L[c]] <- c

        let r_c = grid[c].r;
        let l_c = grid[c].l;
        grid[r_c].l = c;
        grid[l_c].r = c;
    }

    fn partial_solve(
        &mut self,
        partial_solution: &[usize],
    ) -> Result<Vec<usize>, DancingLinksError> {
        let mut partial_solution_nodes = Vec::new();

        for r in partial_solution {
            // convert row into id
            let mut id = None;
            for i in 0..self.grid.len() {
                if let Some(p) = self.grid[i].p {
                    if p.y == *r {
                        id = Some(i);
                        break;
                    }
                }
            }
            // try to hide row
            if let Some(id) = id {
                if self.grid[id].c == id {
                    return Err(DancingLinksError::InternalError {
                        msg: "partial solution found header".to_string(),
                    });
                }

                // traverse columns rightwards
                let mut i = id;
                loop {
                    // add node to solution
                    partial_solution_nodes.push(i);

                    // cover column
                    self.cover(self.grid[i].c);

                    i = self.grid[i].r;

                    if i == id {
                        break;
                    }
                }
            } else {
                return Err(DancingLinksError::InvalidPartialSolution { row: *r });
            }
        }

        Ok(partial_solution_nodes)
    }

    fn search(
        &mut self,
        k: usize,
        solutions: &mut Vec<Vec<usize>>,
        partial_solution: &mut Vec<usize>,
    ) {
        // If the matrix A has no columns, the current partial
        // solution is a valid solution; terminate successfully.
        if self.grid[0].r == 0 {
            // algorithm finished
            solutions.push(partial_solution.clone());
        } else {
            // Otherwise choose a column c (deterministically).
            let c = self.grid[0].r;
            self.cover(c);

            // Choose a row r such that Ar, c = 1 (nondeterministically).
            let mut r = self.grid[c].d;
            while r != c {
                // add R to the partial solution
                partial_solution.push(r);

                // traverse columns rightwards
                let mut j = self.grid[r].r;
                while j != r {
                    // cover column j
                    self.cover(self.grid[j].c);

                    j = self.grid[j].r;
                }

                // search again recursively
                self.search(k + 1, solutions, partial_solution);

                // give up on solution
                partial_solution.pop();

                j = self.grid[r].l;
                while j != r {
                    // uncover column j
                    self.uncover(self.grid[j].c);

                    j = self.grid[j].l;
                }

                r = self.grid[r].d;
            }

            self.uncover(c);
        }
    }

    pub fn solve(
        mut self,
        partial_solution: Option<&[usize]>,
    ) -> Result<Vec<Vec<usize>>, DancingLinksError> {
        let mut solutions = Vec::new();

        let mut partial_solution = match partial_solution {
            Some(partial_solution) => self.partial_solve(partial_solution)?,
            None => Vec::new(),
        };

        self.search(0, &mut solutions, &mut partial_solution);

        for solution in solutions.iter_mut() {
            for node in solution.iter_mut() {
                *node = self.grid[*node].p.unwrap().y;
            }
        }

        if solutions.is_empty() {
            Err(DancingLinksError::NoSolutions)
        } else {
            Ok(solutions)
        }
    }
}
