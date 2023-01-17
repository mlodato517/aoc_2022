pub struct Grid {
    grid: Vec<Box<[u8]>>,
    s_loc: (usize, usize),
    e_loc: (usize, usize),
    visited: Vec<Vec<bool>>,
}
impl Grid {
    pub fn new(grid: &str) -> Self {
        let mut grid: Vec<Box<[u8]>> = grid
            .lines()
            .map(|l| l.trim().to_owned().into_boxed_str().into_boxed_bytes())
            .collect();

        // Record the start and end locations and turn 'S' and 'E' into 'a' and 'z'.
        // TODO We need to allocate to mutate bytes. Are those allocations more or less expensive than
        // coercing 'S' and 'E' in `new_directions` when we check elevation? Could also go straight to
        // bytes at the beginning and split on `b'\n'`.
        let mut s_loc = (0, 0);
        let mut e_loc = (0, 0);
        for (row_idx, row) in grid.iter_mut().enumerate() {
            for (col_idx, cell) in row.iter_mut().enumerate() {
                if *cell == b'S' {
                    s_loc = (row_idx, col_idx);
                    *cell = b'a';
                } else if *cell == b'E' {
                    e_loc = (row_idx, col_idx);
                    *cell = b'z';
                }
            }
        }

        // TODO bitarray
        let visited = vec![vec![false; grid[0].len()]; grid.len()];

        Self {
            grid,
            s_loc,
            e_loc,
            visited,
        }
    }

    /// Get iterator over directions that can be traveled to from the current `row` and `col` in
    /// `grid`. This takes into account elevation and visited-ness.
    fn next_frontier(
        &mut self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let up = (row > 0).then_some((row.wrapping_sub(1), col));
        let down = (row + 1 < self.grid.len()).then_some((row + 1, col));

        let left = (col > 0).then_some((row, col.wrapping_sub(1)));
        let right = (col + 1 < self.grid[row].len()).then_some((row, col + 1));

        [up, down, left, right]
            .into_iter()
            .flatten()
            .filter(move |&(new_row, new_col)| {
                // NOTE we check that we don't go too far _down_. That's because we'll go from the
                // back of the graph to the front.
                let scalable = self.grid[new_row][new_col] >= self.grid[row][col] - 1;
                let eligible = scalable && !self.visited[new_row][new_col];
                if eligible {
                    self.visited[new_row][new_col] = true;
                }
                eligible
            })
    }

    /// Find the shortest path to the end location from a location identified by `done`.
    pub fn shortest_path_length<F>(&mut self, done: F) -> u64
    where
        F: Fn(&Self, usize, usize) -> bool,
    {
        //TODO Try our hands at A* or a BFS from the start and end that meet
        let mut curr_frontier = Vec::new();
        let mut next_frontier = vec![self.e_loc];
        self.visited[self.e_loc.0][self.e_loc.1] = true;

        let mut path_length = 0;
        'outer: loop {
            std::mem::swap(&mut curr_frontier, &mut next_frontier);
            for (row, col) in curr_frontier.drain(..) {
                if done(self, row, col) {
                    break 'outer path_length;
                }
                next_frontier.extend(self.next_frontier(row, col));
            }

            path_length += 1;
        }
    }

    pub fn start(&self) -> (usize, usize) {
        self.s_loc
    }

    pub fn elevation(&self, row: usize, col: usize) -> u8 {
        self.grid[row][col]
    }
}
