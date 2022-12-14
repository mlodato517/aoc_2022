use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
struct Direction {
    x: i32,
    y: i32,
}

pub fn part1(input: &str) -> u64 {
    let mut unique_positions = HashSet::new();

    let mut head_pos = Point::default();
    let mut tail_pos = Point::default();

    unique_positions.insert(tail_pos);

    for line in input.lines() {
        let (direction, distance) = parse_movement(line);

        // Is there a better way to do this? We could move head all at once but we'd (probably)
        // need to step tail one-at-a-time to record all the positions.
        for _ in 0..distance {
            head_pos.x += direction.x;
            head_pos.y += direction.y;

            if head_pos.x.abs_diff(tail_pos.x) > 1 {
                tail_pos.x += direction.x;

                // Move the tail diagonally if we need to. Could special case this.
                tail_pos.y += head_pos.y - tail_pos.y;
            } else if head_pos.y.abs_diff(tail_pos.y) > 1 {
                // Move the tail diagonally if we need to. Could special case this.
                tail_pos.x += head_pos.x - tail_pos.x;

                tail_pos.y += direction.y;
            }

            unique_positions.insert(tail_pos);
        }
    }

    unique_positions.len() as u64
}

// fn print_grid(head: Point, tail: Point) {
//     for y in 0..6 {
//         for x in 0..6 {
//             if x == head.x && 5 - y == head.y {
//                 print!("H");
//             } else if x == tail.x && 5 - y == tail.y {
//                 print!("T");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
//     println!();
// }

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

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let file = "R 4\n\
                        U 4\n\
                        L 3\n\
                        D 1\n\
                        R 4\n\
                        D 1\n\
                        L 5\n\
                        R 2";
            assert_eq!(part1(file), 13);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part1(file), 5619);
        }
    }
}
