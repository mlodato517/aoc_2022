use wall::{HorizontalWall, Wall, WallRef, Walls};

mod wall;

pub fn part1(input: &str) -> u64 {
    let walls = parse_walls(input);
    let walls = Walls::new(walls);

    sand_count(walls)
}

fn parse_coordinate(pair: &str) -> (i64, i64) {
    let mut coords = pair.split(',');
    let x = coords.next().expect("Missing X coordinate");
    let x = x.parse().expect("Invalid X coordinate");
    let y = coords.next().expect("Missing Y coordinate");
    let y = y.parse().expect("Invalid Y coordinate");

    (x, y)
}

fn parse_walls(input: &str) -> Vec<Wall> {
    input
        .lines()
        .flat_map(|wall| {
            let mut split = wall.split(" -> ");
            let first_pair = split.next().expect("Line has no start!");
            let mut wall_start = parse_coordinate(first_pair);
            split.map(move |coordinate| {
                let wall_end = parse_coordinate(coordinate);
                let wall = Wall::new(wall_start, wall_end);
                wall_start = wall_end;
                wall
            })
        })
        .collect()
}

fn sand_count(mut walls: Walls) -> u64 {
    const SAND_FALLING_COORDINATE: (i64, i64) = (500, 0);
    let mut sand_count = 0;

    let mut sand_position = SAND_FALLING_COORDINATE;
    let mut sand_positions = Vec::new();
    loop {
        // This sand fell off the map - no new sand can fall.
        let Some(wall) = walls.intersection_for_vertical_ray(sand_position) else {
            return sand_count;
        };

        sand_position.1 = wall.top() - 1;

        // Attempt to cascade left and then right. If we cascade then we loop again to go back to
        // falling straight down. If we come to a rest, we generate another sand.
        let down_left = (sand_position.0 - 1, sand_position.1 + 1);
        let wall_blocking = walls.contains(down_left);
        if !wall_blocking {
            sand_positions.push(sand_position);
            sand_position = down_left;
            continue;
        }

        let down_right = (sand_position.0 + 1, sand_position.1 + 1);
        let wall_blocking = walls.contains(down_right);
        if !wall_blocking {
            sand_positions.push(sand_position);
            sand_position = down_right;
            continue;
        }

        // Can't fall left or right - coming to rest here.
        sand_count += 1;

        // This sand is blocking up the entrance - no new sand can fall.
        if sand_position.1 == 0 {
            return sand_count;
        }

        match wall {
            WallRef::Vertical(v) => v.y_top -= 1,
            WallRef::Horizontal(_) => walls.add_sand(sand_position),
        }

        // The next bit of sand will fall at least to the place this sand just came from.
        sand_position = sand_positions.pop().unwrap_or(SAND_FALLING_COORDINATE);
    }
}

pub fn part2(input: &str) -> u64 {
    let mut walls = parse_walls(input);

    let lowest_y_value = walls
        .iter()
        .map(|wall| wall.bottom())
        .max()
        .expect("No walls!");

    // Infinite floor
    walls.push(Wall::Horizontal(HorizontalWall {
        y: lowest_y_value + 2,
        x_left: i64::MIN,
        x_right: i64::MAX - 1,
    }));

    let walls = Walls::new(walls);

    sand_count(walls)
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
        fn simple_vertical_walls() {
            let example = "400,0 -> 300,0";
            assert_eq!(part1(example), 0);

            let example = "500,10 -> 500,20";
            assert_eq!(part1(example), 0);
        }

        #[test]
        fn simple_horizontal_walls() {
            let example = "501,3 -> 502,3";
            assert_eq!(part1(example), 0);

            let example = "500,3 -> 501,3";
            assert_eq!(part1(example), 0);

            let example = "499,3 -> 501,3";
            assert_eq!(part1(example), 1);

            let example = "498,3 -> 502,3";
            assert_eq!(part1(example), 4);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 757);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part2(EXAMPLE), 93);
        }

        #[test]
        fn my_input() {
            assert_eq!(part2(INPUT), 24943);
        }
    }
}
