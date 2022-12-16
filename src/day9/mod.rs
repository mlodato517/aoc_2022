use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

struct Rope<const N: usize> {
    segments: [Point; N],
}

impl<const N: usize> fmt::Debug for Rope<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[")?;
        for point in &self.segments {
            write!(f, "({},{}),", point.x, point.y)?; // screw trailing commas
        }
        f.write_str("]")
    }
}

// Explicit implementation because `N` isn't necessarily <= 32. See
// https://doc.rust-lang.org/std/primitive.array.html.
impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self {
            segments: [Point::default(); N],
        }
    }
}

impl<const N: usize> Rope<N> {
    fn step(&mut self, direction: Direction) {
        assert!(N > 0);

        self.segments[0].x += direction.x;
        self.segments[0].y += direction.y;

        for i in 1..N {
            let prev_pos = self.segments[i - 1];
            let cur_pos = &mut self.segments[i];

            let dx = prev_pos.x - cur_pos.x;
            let dy = prev_pos.y - cur_pos.y;

            let dx_mag = dx.abs();
            let dy_mag = dy.abs();

            if dx_mag <= 1 && dy_mag <= 1 {
                continue;
            }

            // TODO make branchless
            match dx_mag.cmp(&dy_mag) {
                Ordering::Greater => {
                    cur_pos.x += dx - dx.signum();
                    cur_pos.y = prev_pos.y;
                }
                Ordering::Equal => {
                    cur_pos.x += dx - dx.signum();
                    cur_pos.y += dy - dy.signum();
                }
                Ordering::Less => {
                    cur_pos.x = prev_pos.x;
                    cur_pos.y += dy - dy.signum();
                }
            }
        }
    }

    fn tail(&self) -> Point {
        assert!(N > 0);

        self.segments[N - 1]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Clone, Copy)]
struct Direction {
    x: i32,
    y: i32,
}

pub fn part1(input: &str) -> u64 {
    let mut unique_positions = HashSet::new();

    let mut rope = Rope::<2>::default();

    unique_positions.insert(rope.tail());

    for line in input.lines() {
        let (direction, distance) = parse_movement(line);

        // Is there a better way to do this? We could move head all at once but we'd (probably)
        // need to step tail one-at-a-time to record all the positions.
        for _ in 0..distance {
            rope.step(direction);
            unique_positions.insert(rope.tail());
        }
    }

    unique_positions.len() as u64
}

fn parse_movement(line: &str) -> (Direction, i32) {
    let mut split = line.split(' ');
    let direction = split.next().expect("Should have direction");
    let distance = split.next().expect("Should have distance");

    let direction = match direction {
        "R" => Direction { x: 1, y: 0 },
        "U" => Direction { x: 0, y: 1 },
        "L" => Direction { x: -1, y: 0 },
        "D" => Direction { x: 0, y: -1 },
        _ => panic!("Invalid direction"),
    };
    let distance: i32 = distance.parse().expect("Distance should be a number");

    (direction, distance)
}

pub fn part2(input: &str) -> u64 {
    let mut unique_positions = HashSet::new();

    let mut rope = Rope::<10>::default();

    unique_positions.insert(rope.tail());

    for line in input.lines() {
        let (direction, distance) = parse_movement(line);

        for _ in 0..distance {
            rope.step(direction);
            unique_positions.insert(rope.tail());
        }
    }

    unique_positions.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "R 4\n\
                            U 4\n\
                            L 3\n\
                            D 1\n\
                            R 4\n\
                            D 1\n\
                            L 5\n\
                            R 2";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1(EXAMPLE1), 13);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 5619);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example_1() {
            assert_eq!(part2(EXAMPLE1), 1);
        }

        #[test]
        fn example_2() {
            let file = "R 5\n\
                        U 8\n\
                        L 8\n\
                        D 3\n\
                        R 17\n\
                        D 10\n\
                        L 25\n\
                        U 20";
            assert_eq!(part2(file), 36);
        }

        #[test]
        fn my_input() {
            assert_eq!(part2(INPUT), 2376);
        }
    }
}
