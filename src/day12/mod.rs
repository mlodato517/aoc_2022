pub fn part1(input: &str) -> u64 {
    let mut grid: Vec<Box<[u8]>> = input
        .lines()
        .map(|l| l.trim().to_owned().into_boxed_str().into_boxed_bytes())
        .collect();

    // Record the start and end locations and turn 'S' and 'E' into 'a' and 'z'.
    // TODO We need to allocate to mutate bytes. Are those allocations more or less expensive than
    // coercing 'S' and 'E' in `new_directions` when we check elevation?
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row_idx, row) in grid.iter_mut().enumerate() {
        for (col_idx, cell) in row.iter_mut().enumerate() {
            if *cell == b'S' {
                start = (row_idx, col_idx);
                *cell = b'a';
            } else if *cell == b'E' {
                end = (row_idx, col_idx);
                *cell = b'z';
            }
        }
    }

    // TODO bitarray
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[start.0][start.1] = true;

    //TODO Try our hands at A* or a BFS from the start and end that meet
    let mut curr_frontier = Vec::new();
    let mut next_frontier = vec![start];
    let mut path_length = 0;
    'outer: loop {
        std::mem::swap(&mut curr_frontier, &mut next_frontier);
        for (row, col) in curr_frontier.drain(..) {
            if (row, col) == end {
                break 'outer path_length;
            }
            for (new_row, new_col) in new_directions(&grid, row, col) {
                if !visited[new_row][new_col] {
                    visited[new_row][new_col] = true;
                    next_frontier.push((new_row, new_col));
                }
            }
        }

        path_length += 1;
    }
}

/// Get iterator over directions that can be traveled to from the current `row` and `col` in
/// `grid`. This takes into account elevation as well as bounds checking.
fn new_directions(
    grid: &[Box<[u8]>],
    row: usize,
    col: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let curr_elevation = grid[row][col];

    // NOTE because of the non-laziness of the subtraction in the `then_some`, we need to run in
    // release mode so we can ignore the underflowing subtraction. Could use `then(|| ...)` but...
    let up = (row > 0 && grid[row - 1][col] <= curr_elevation + 1).then_some((row - 1, col));
    let down = (row + 1 < grid.len() && grid[row + 1][col] <= curr_elevation + 1)
        .then_some((row + 1, col));

    let left = (col > 0 && grid[row][col - 1] <= curr_elevation + 1).then_some((row, col - 1));
    let right = (col + 1 < grid[row].len() && grid[row][col + 1] <= curr_elevation + 1)
        .then_some((row, col + 1));

    let new_directions = [up, down, left, right];

    new_directions.into_iter().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Sabqponm
                           abcryxxl
                           accszExk
                           acctuvwj
                           abdefghi";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1(EXAMPLE), 31);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 497);
        }
    }
}
