use std::collections::HashSet;

// Not sure but I think y values can't be negative. I should check this against my input though it
// doesn't really matter.
pub fn part1((input, row): (&str, u64)) -> u64 {
    let mut dead_zones_in_row = Vec::new();
    let mut beacons_in_row = HashSet::new();
    for line in input.lines() {
        let (sensor, beacon) = parse_sensors_and_beacons(line);
        if beacon.1 == row as i64 {
            beacons_in_row.insert(beacon.0);
        }
        let dist = sensor.0.abs_diff(beacon.0) as i64 + sensor.1.abs_diff(beacon.1) as i64;
        let vertical_dist_to_row = sensor.1.abs_diff(row as i64) as i64;
        if vertical_dist_to_row > dist {
            // Sensor and beacon are close enough together and far enough from the target row that
            // they don't provide any information.
            continue;
        }

        // The distance from the target row determines how wide the dead zone is.
        let width = dist - vertical_dist_to_row;
        dead_zones_in_row.push(sensor.0 - width..=sensor.0 + width);
    }

    // First, consolidate ranges. Itertools has something for this... Coalesce?
    dead_zones_in_row.sort_unstable_by_key(|range| *range.start());
    let mut consolidated_zones = Vec::new();

    let range = &dead_zones_in_row[0];
    let mut range = (*range.start(), *range.end());
    for next_range in dead_zones_in_row.into_iter().skip(1) {
        if *next_range.start() <= range.1 {
            if *next_range.end() > range.1 {
                range.1 = *next_range.end();
            }
        } else {
            consolidated_zones.push(range);
            range = (*next_range.start(), *next_range.end());
        }
    }
    consolidated_zones.push(range);

    // Second, Get the total width covered
    let mut count = consolidated_zones
        .iter()
        .map(|(start, end)| (*end - *start + 1) as u64)
        .sum();

    // Finally, subtract known beacons
    // I think the zones here are still sorted so this could be a binary search but :shrug:
    for beacon in beacons_in_row {
        let counted = consolidated_zones
            .iter()
            .any(|(start, end)| *start <= beacon && beacon <= *end);
        if counted {
            count -= 1;
        }
    }

    count
}

fn parse_sensors_and_beacons(line: &str) -> ((i64, i64), (i64, i64)) {
    let mut split = line.split(['=', ',', ':']);
    let _sensor_prefix = split.next();
    let sensor_x = split.next().expect("Missing sensor X coordinate");
    let _coordinate_delimiter = split.next();
    let sensor_y = split.next().expect("Missing sensor Y coordinate");

    let _beacon_prefix = split.next();
    let beacon_x = split.next().expect("Missing beacon X coordinate");
    let _coordinate_delimiter = split.next();
    let beacon_y = split.next().expect("Missing beacon Y coordinate");

    let sensor_x = sensor_x.parse().expect("Invalid sensor X");
    let sensor_y = sensor_y.parse().expect("Invalid sensor Y");
    let beacon_x = beacon_x.parse().expect("Invalid beacon X");
    let beacon_y = beacon_y.parse().expect("Invalid beacon Y");

    ((sensor_x, sensor_y), (beacon_x, beacon_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1((EXAMPLE, 10)), 26);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1((INPUT, 2_000_000)), 5878678);
        }
    }
}
