#[derive(Debug)]
enum StraightLine {
    Horizontal { y: u64, x_left: u64, x_right: u64 },
    Vertical { x: u64, y_low: u64, y_high: u64 },
}

impl StraightLine {
    fn new(start: (u64, u64), end: (u64, u64)) -> Self {
        // X positions are the same - this is vertical
        if start.0 == end.0 {
            // Smaller y values are higher
            if start.1 < end.1 {
                Self::Vertical {
                    x: start.0,
                    y_low: end.1,
                    y_high: start.1,
                }
            } else {
                Self::Vertical {
                    x: start.0,
                    y_low: start.1,
                    y_high: end.1,
                }
            }
        } else {
            // Better be horizontal
            // Smaller x values are left
            if start.0 < end.0 {
                Self::Horizontal {
                    y: start.1,
                    x_left: start.0,
                    x_right: end.0,
                }
            } else {
                Self::Horizontal {
                    y: start.1,
                    x_left: end.0,
                    x_right: start.0,
                }
            }
        }
    }

    /// Determine the y coordinate at which the sand will stop at if it starts falling from
    /// `point`.
    fn stops_falling_sand_at(&self, point: (u64, u64)) -> Option<u64> {
        // Remember Y values increase as we go down!
        match self {
            Self::Horizontal { y, x_left, x_right } => {
                let point_is_above = point.1 < *y;
                let point_in_x_range = (*x_left..=*x_right).contains(&point.0);
                (point_is_above && point_in_x_range).then_some(*y - 1)
            }
            Self::Vertical { x, y_high, .. } => {
                let point_is_above = point.1 < *y_high;
                let point_in_x_range = *x == point.0;
                (point_is_above && point_in_x_range).then_some(*y_high - 1)
            }
        }
    }

    fn contains(&self, point: (u64, u64)) -> bool {
        match self {
            Self::Horizontal { y, x_left, x_right } => {
                *y == point.1 && (*x_left..=*x_right).contains(&point.0)
            }
            Self::Vertical { x, y_high, y_low } => {
                *x == point.0 && (*y_high..=*y_low).contains(&point.1)
            }
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut lines: Vec<_> = input
        .lines()
        .flat_map(|line| {
            let mut split = line.split(" -> ");
            let first_pair = split.next().expect("Line has no start!");
            let mut line_start = parse_coordinate(first_pair);
            split.map(move |coordinate| {
                let line_end = parse_coordinate(coordinate);
                let line = StraightLine::new(line_start, line_end);
                line_start = line_end;
                line
            })
        })
        .collect();

    // TODO Preprocess lines for faster lookups later. The two operations we need are:
    //   1. given a sand starting position, what's the first rock it hits?
    //   2. given a position, is there a rock there?
    //
    // If we could assume that two walls touching implied both contain the intersection point then
    // we could trivially handle `2` just by looking at what kind of thing we collided with. Note
    // that, when sand comes to rest, we'd want to preferentially select horizontal walls. But
    // let's not assume that.
    //
    // One idea is that we could keep the vertical walls in a separate list. We could store these
    // in a `HashMap` keyed by X coordinate and use them to quickly find the highest vertical
    // wall. We should make sure we call rock walls with a single rock "vertical".
    //
    // This doesn't help quickly finding the highest horizontal wall. One fairly lame short circuit
    // is to sort by Y coordinate and go in order so we can stop at the first one but it'd be much
    // better if we could do something else.
    //
    // I'm not sure how many walls there are and what their lengths are but we could break up a
    // single horizontal wall into N vertical walls. This could be a nightmare if there's one
    // _very_ long wall but a reasonable optimization otherwise.

    let mut sand_count = 0;

    const SAND_FALLING_COORDINATE: (u64, u64) = (500, 0);
    let mut sand_position = SAND_FALLING_COORDINATE;
    loop {
        // TODO preprocess lines so this is fast. Maybe BTree something?
        let sand_height = lines
            .iter()
            .filter_map(|line| line.stops_falling_sand_at(sand_position))
            .min();

        let Some(sand_height) = sand_height else {
            return sand_count;
        };
        sand_position.1 = sand_height;

        // Attempt to cascade left and then right. If we cascade then we loop again to go back to
        // falling straight down. If we come to a rest, we reset the sand position.
        let down_left = (sand_position.0 - 1, sand_position.1 + 1);
        // TODO preprocess lines so this is fast
        let wall_blocking = lines.iter().any(|line| line.contains(down_left));
        if !wall_blocking {
            sand_position = down_left;
        } else {
            let down_right = (sand_position.0 + 1, sand_position.1 + 1);
            let wall_blocking = lines.iter().any(|line| line.contains(down_right));
            if !wall_blocking {
                sand_position = down_right;
            } else {
                // Can't fall left or right - coming to rest here.
                sand_count += 1;

                // This sand now acts like a rock. Probably horribly inefficient. Maybe keep track
                // of the boundary?
                lines.push(StraightLine::new(sand_position, sand_position));

                // I shouldn't have to start over at the beginning I don't think...
                sand_position = SAND_FALLING_COORDINATE;
            }
        }
    }
}

fn parse_coordinate(pair: &str) -> (u64, u64) {
    let mut coords = pair.split(',');
    let x = coords.next().expect("Missing X coordinate");
    let x = x.parse().expect("Invalid X coordinate");
    let y = coords.next().expect("Missing Y coordinate");
    let y = y.parse().expect("Invalid Y coordinate");

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6\n\
                           503,4 -> 502,4 -> 502,9 -> 494,9";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1(EXAMPLE), 24);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 24);
        }
    }
}
