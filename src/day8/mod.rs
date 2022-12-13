pub fn part1(input: &str) -> u64 {
    // Collect to a Vec so we can iterate in reverse later.
    let heights: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let num_rows = heights.len();
    let num_cols = heights[0].len();

    let mut visible = vec![vec![false; num_cols]; num_rows];
    let mut count = 0;

    for (height_row, visible_row) in heights.iter().zip(&mut visible) {
        let from_left = height_row.iter().copied().zip(&mut *visible_row);
        count += count_visible(from_left);

        let from_right = height_row.iter().copied().zip(&mut *visible_row).rev();
        count += count_visible(from_right);
    }

    // TODO this is cache-inefficient. Combine these loops or maybe transpose the matrix.
    // Also, is it better to go column-by-column and then reverse or column-then-reverse?
    for col in 0..num_cols {
        let from_top = heights
            .iter()
            .map(|row| row[col])
            .zip(visible.iter_mut().map(|row| &mut row[col]));
        count += count_visible(from_top);

        let from_bottom = heights
            .iter()
            .map(|row| row[col])
            .zip(visible.iter_mut().map(|row| &mut row[col]))
            .rev();
        count += count_visible(from_bottom);
    }

    count
}

fn count_visible<'a, I>(trees_and_slots: I) -> u64
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

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let file = "30373\n\
            25512\n\
            65332\n\
            33549\n\
            35390";

            assert_eq!(part1(file), 21);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part1(file), 1870);
        }
    }
}
