#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Node {
    l: usize,         // left
    r: usize,         // right
    u: usize,         // up
    d: usize,         // down
    c: usize,         // column
    p: Option<Point>, // node position
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
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

#[derive(Debug)]
pub enum DancingLinksError {
    InvalidMatrixSize { expected: usize, got: usize },
    InternalError { msg: String },
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
            DancingLinksError::InternalError { msg } => {
                write!(f, "internal error occurred: {msg}!")
            }
        }
    }
}

impl std::error::Error for DancingLinksError {}

pub struct DancingLinks {
    grid: Vec<Node>,
}

impl std::fmt::Display for DancingLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write as _;

        let mut out = String::new();

        for (i, node) in self.grid.iter().enumerate() {
            if let Some(point) = &node.p {
                writeln!(
                    out,
                    "Node {i} : L[{}] R[{}] U[{}] D[{}] C[{}] ({}, {})",
                    node.l, node.r, node.u, node.d, node.c, point.x, point.y
                )
                .unwrap();
            } else {
                writeln!(
                    out,
                    "Node {i} : L[{}] R[{}] U[{}] D[{}] C[{}] (N/A)",
                    node.l, node.r, node.u, node.d, node.c
                )
                .unwrap();
            }
        }

        write!(f, "{}", out)
    }
}

impl DancingLinks {
    pub fn new(matrix: &Vec<bool>, width: usize, height: usize) -> Result<Self, DancingLinksError> {
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
                let dlx = DancingLinks { grid };

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
}
