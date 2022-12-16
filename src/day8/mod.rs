pub fn part1(input: &str) -> u64 {
    // Collect to a Vec so we can iterate in reverse later.
    let heights: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let num_rows = heights.len();
    let num_cols = heights[0].len();

    let mut visible = vec![vec![false; num_cols]; num_rows];
    let mut count = 0;

    for (height_row, visible_row) in heights.iter().zip(&mut visible) {
        let from_left = height_row.iter().copied().zip(&mut *visible_row);
        count += count_visible_from_ground(from_left);

        let from_right = height_row.iter().copied().zip(&mut *visible_row).rev();
        count += count_visible_from_ground(from_right);
    }

    // TODO this is cache-inefficient. Combine these loops or maybe transpose the matrix.
    // Also, is it better to go column-by-column and then reverse or column-then-reverse?
    for col_idx in 0..num_cols {
        let from_top = heights
            .iter()
            .map(|row| row[col_idx])
            .zip(visible.iter_mut().map(|row| &mut row[col_idx]));
        count += count_visible_from_ground(from_top);

        let from_bottom = heights
            .iter()
            .map(|row| row[col_idx])
            .zip(visible.iter_mut().map(|row| &mut row[col_idx]))
            .rev();
        count += count_visible_from_ground(from_bottom);
    }

    count
}

fn count_visible_from_ground<'a, I>(trees_and_slots: I) -> u64
where
    I: IntoIterator<Item = (u8, &'a mut bool)>,
{
    // Not obviously nicer than a for loop but it's always fun to play with `scan`
    trees_and_slots
        .into_iter()
        .scan(None, |tallest, (tree, visible_slot)| {
            let is_visible = tallest.map_or(true, |t| tree > t);
            if !is_visible {
                return Some(false);
            }
            *tallest = Some(tree);
            let updated = !(*visible_slot);
            *visible_slot = true;
            Some(updated)
        })
        .filter(|updated| *updated)
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    // Collect to a Vec so we can iterate in reverse later.
    let heights: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let num_rows = heights.len();
    let num_cols = heights[0].len();

    let mut scores = vec![vec![1; num_cols]; num_rows];
    let mut max_score = 0;

    // There's gotta be an O(1) way to do this per row...
    //
    // A slightly better O(n) way would be to, for each tree you can see, see how many
    // trees it can see (because you should be able to see those too). That cuts things
    // down by some constant factor but is still O(n) and probably not enough value for the
    // work right now.

    for (height_row, score_row) in heights.iter().zip(&mut scores) {
        for col_idx in 0..height_row.len() {
            let tree = height_row[col_idx];

            // Update scenic score from the left
            let prev_trees = (0..col_idx).rev().map(|row_idx| height_row[row_idx]);
            let visible_trees = count_visible_from_tree(tree, prev_trees);
            score_row[col_idx] *= visible_trees;

            // Update scenic score from the right
            let prev_trees = &height_row[col_idx + 1..];
            let visible_trees = count_visible_from_tree(tree, prev_trees.iter().copied());
            score_row[col_idx] *= visible_trees;
        }
    }

    // TODO this is cache-inefficient. Combine these loops or maybe transpose the matrix.
    // Also, is it better to go column-by-column and then reverse or column-then-reverse?
    for col_idx in 0..num_cols {
        for row_idx in 0..num_rows {
            let tree = heights[row_idx][col_idx];

            // Update scenic score from the top
            let prev_trees = (0..row_idx).rev().map(|row_idx| heights[row_idx][col_idx]);
            let visible_trees = count_visible_from_tree(tree, prev_trees);
            scores[row_idx][col_idx] *= visible_trees;

            // Update scenic score from the bottom
            let prev_trees = (row_idx + 1..num_rows).map(|row_idx| heights[row_idx][col_idx]);
            let visible_trees = count_visible_from_tree(tree, prev_trees);
            scores[row_idx][col_idx] *= visible_trees;

            max_score = max_score.max(scores[row_idx][col_idx]);
        }
    }

    max_score
}

fn count_visible_from_tree<I>(tree: u8, prev_trees: I) -> u64
where
    I: IntoIterator<Item = u8>,
{
    let mut visible_trees = 0;
    for prev_tree in prev_trees {
        visible_trees += 1;
        if tree <= prev_tree {
            break;
        }
    }

    visible_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "30373\n\
                           25512\n\
                           65332\n\
                           33549\n\
                           35390";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1(EXAMPLE), 21);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 1870);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part2(EXAMPLE), 8);
        }

        #[test]
        fn my_input() {
            assert_eq!(part2(INPUT), 517440);
        }
    }
}
