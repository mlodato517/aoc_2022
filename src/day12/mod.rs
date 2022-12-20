mod grid;

use grid::Grid;

pub fn part1(input: &str) -> u64 {
    let mut grid = Grid::new(input);
    let done = |grid: &Grid, row, col| (row, col) == grid.start();
    grid.shortest_path_length(done)
}

pub fn part2(input: &str) -> u64 {
    let mut grid = Grid::new(input);
    let done = |grid: &Grid, row, col| grid.elevation(row, col) == b'a';
    grid.shortest_path_length(done)
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

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part2(EXAMPLE), 29);
        }

        #[test]
        fn my_input() {
            assert_eq!(part2(INPUT), 492);
        }
    }
}
