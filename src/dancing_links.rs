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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Data {
    Point(Point),
    Size(usize),
    Root,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Node {
    l: usize, // left
    r: usize, // right
    u: usize, // up
    d: usize, // down
    c: usize, // column
    x: Data,  // extended data
}

impl Node {
    fn new(id: usize, x: Data) -> Self {
        Self {
            l: id,
            r: id,
            u: id,
            d: id,
            c: id,
            x,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.x {
            Data::Point(p) => {
                write!(
                    f,
                    "L[{}] R[{}] U[{}] D[{}] C[{}] ({}, {})",
                    self.l, self.r, self.u, self.d, self.c, p.x, p.y
                )
            }
            Data::Size(s) => {
                write!(
                    f,
                    "L[{}] R[{}] U[{}] D[{}] C[{}] S[{}]",
                    self.l, self.r, self.u, self.d, self.c, s
                )
            }
            Data::Root => {
                write!(
                    f,
                    "L[{}] R[{}] U[{}] D[{}] C[{}] (root)",
                    self.l, self.r, self.u, self.d, self.c
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidMatrixSize { expected: usize, got: usize },
    InvalidPartialSolution { row: usize },
    InternalError { msg: String },
    NoSolutions,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidMatrixSize { expected, got } => {
                write!(
                    f,
                    "invalid matrix dimensions, expected {expected} elements, got {got}!"
                )
            }
            Error::InvalidPartialSolution { row } => {
                write!(
                    f,
                    "invalid partial solution entered: row {row} is part of header!"
                )
            }
            Error::InternalError { msg } => {
                write!(f, "internal error occurred: {msg}!")
            }
            Error::NoSolutions => {
                write!(f, "no solutions found!")
            }
        }
    }
}

impl std::error::Error for Error {}

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
    pub fn new(matrix: &[bool], width: usize, height: usize) -> Result<Self, Error> {
        // check that dimensions are valid
        if matrix.len() == width * height {
            // count number of nodes
            let num_ones: usize = matrix.iter().map(|b| if *b { 1 } else { 0 }).sum();

            // root + columns + nodes
            let mut grid = Vec::with_capacity(1 + width + num_ones);

            // create root node
            let root_id = grid.len();
            grid.push(Node::new(root_id, Data::Root));

            // create column nodes
            let mut rcol_id = root_id;
            for i in 0..width {
                let col_id = grid.len();
                grid.push(Node::new(col_id, Data::Size(0)));

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
                        grid.push(Node::new(row_id, Data::Point(Point { x: i, y: j })));

                        // attach column id
                        grid[row_id].c = col_id;

                        // knit into column
                        grid[col_id].u = row_id;
                        grid[row_id].d = col_id;
                        grid[trow_id].d = row_id;
                        grid[row_id].u = trow_id;
                        trow_id = row_id;

                        // increase column size
                        match grid[col_id].x {
                            Data::Size(ref mut c) => *c += 1,
                            _ => {
                                return Err(Error::InternalError {
                                    msg: "row object has invalid column header".to_string(),
                                })
                            }
                        }

                        // knit into start of row
                        let mut row_start: Option<usize> = None;
                        for (cell_id, cell) in grid.iter().enumerate() {
                            if let Data::Point(point) = &cell.x {
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
                                if let Data::Point(point) = &cell.x {
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
                                return Err(Error::InternalError {
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
                Err(Error::InternalError {
                    msg: "incorrect grid length".to_owned(),
                })
            }
        } else {
            Err(Error::InvalidMatrixSize {
                expected: width * height,
                got: matrix.len(),
            })
        }
    }

    fn cover(&mut self, c: usize) -> Result<(), Error> {
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

                // Step 5: Decrement column size
                let c_j = grid[j].c;
                match grid[c_j].x {
                    Data::Size(ref mut s) => *s -= 1,
                    _ => {
                        return Err(Error::InternalError {
                            msg: "tried covering non-column object".to_string(),
                        })
                    }
                }

                j = grid[j].r;
            }

            i = grid[i].d
        }

        Ok(())
    }

    fn uncover(&mut self, c: usize) -> Result<(), Error> {
        let grid = &mut self.grid;

        // Step 1: Iterate through rows in column (upwards)
        let mut i = grid[c].u;
        while i != c {
            // Step 2: Iterate through cells in row (leftwards)
            let mut j = grid[i].l;
            while j != i {
                // Step 3: Increment column size
                let c_j = grid[j].c;
                match grid[c_j].x {
                    Data::Size(ref mut s) => *s += 1,
                    _ => {
                        return Err(Error::InternalError {
                            msg: "tried uncovering non-column object".to_string(),
                        })
                    }
                }

                // Step 4: Unhide cells
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
        // Step 5: Unhide column
        // L[R[c]] <- c
        // R[L[c]] <- c

        let r_c = grid[c].r;
        let l_c = grid[c].l;
        grid[r_c].l = c;
        grid[l_c].r = c;

        Ok(())
    }

    fn partial_solve(&mut self, partial_solution: &[usize]) -> Result<Vec<usize>, Error> {
        let mut partial_solution_nodes = Vec::new();

        for r in partial_solution {
            // convert row into id
            let mut id = None;
            for i in 0..self.grid.len() {
                if let Data::Point(p) = self.grid[i].x {
                    if p.y == *r {
                        id = Some(i);
                        break;
                    }
                }
            }
            // try to hide row
            if let Some(id) = id {
                if self.grid[id].c == id {
                    return Err(Error::InternalError {
                        msg: "partial solution found header".to_string(),
                    });
                }

                // traverse columns rightwards
                let mut i = id;
                loop {
                    // add node to solution
                    partial_solution_nodes.push(i);

                    // cover column
                    self.cover(self.grid[i].c)?;

                    i = self.grid[i].r;

                    if i == id {
                        break;
                    }
                }
            } else {
                return Err(Error::InvalidPartialSolution { row: *r });
            }
        }

        Ok(partial_solution_nodes)
    }

    fn search(
        &mut self,
        k: usize,
        solutions: &mut Vec<Vec<usize>>,
        partial_solution: &mut Vec<usize>,
    ) -> Result<(), Error> {
        // If the matrix A has no columns, the current partial
        // solution is a valid solution; terminate successfully.
        if self.grid[0].r == 0 {
            // algorithm finished
            solutions.push(partial_solution.clone());
        } else {
            // Otherwise choose a column c (deterministically).
            let mut c = self.grid[0].r;
            let mut s = usize::MAX;
            let mut nc = c;
            while nc != 0 {
                let ns = match self.grid[c].x {
                    Data::Size(s) => s,
                    _ => {
                        return Err(Error::InternalError {
                            msg: "traversed non-column object while calculating minimum"
                                .to_string(),
                        });
                    }
                };

                if ns < s {
                    s = ns;
                    c = nc;
                }

                nc = self.grid[nc].r;
            }

            self.cover(c)?;

            // Choose a row r such that Ar, c = 1 (nondeterministically).
            let mut r = self.grid[c].d;
            while r != c {
                // add R to the partial solution
                partial_solution.push(r);

                // traverse columns rightwards
                let mut j = self.grid[r].r;
                while j != r {
                    // cover column j
                    self.cover(self.grid[j].c)?;

                    j = self.grid[j].r;
                }

                // search again recursively
                self.search(k + 1, solutions, partial_solution)?;

                // give up on solution
                partial_solution.pop();

                j = self.grid[r].l;
                while j != r {
                    // uncover column j
                    self.uncover(self.grid[j].c)?;

                    j = self.grid[j].l;
                }

                r = self.grid[r].d;
            }

            self.uncover(c)?;
        }

        Ok(())
    }

    pub fn solve(mut self, partial_solution: Option<&[usize]>) -> Result<Vec<Vec<usize>>, Error> {
        let mut solutions = Vec::new();

        let mut partial_solution = match partial_solution {
            Some(partial_solution) => self.partial_solve(partial_solution)?,
            None => Vec::new(),
        };

        self.search(0, &mut solutions, &mut partial_solution)?;

        for solution in solutions.iter_mut() {
            for node in solution.iter_mut() {
                match self.grid[*node].x {
                    Data::Point(p) => {
                        *node = p.y;
                    }
                    _ => {
                        return Err(Error::InternalError {
                            msg: "found non-row object in solution".to_owned(),
                        });
                    }
                }
            }
        }

        if solutions.is_empty() {
            Err(Error::NoSolutions)
        } else {
            Ok(solutions)
        }
    }
}
