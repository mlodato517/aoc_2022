use std::ops::RangeInclusive;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| {
            let (first_elf_range, second_elf_range) = elf_ranges(line);
            is_superset(&first_elf_range, &second_elf_range)
                || is_superset(&second_elf_range, &first_elf_range)
        })
        .count() as u64
}

fn elf_ranges(line: &str) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    let bytes = line.as_bytes();
    let mut start = 0;
    let mut end = 0;
    while bytes[end] != b'-' {
        end += 1;
    }
    let first_start = line[start..end].parse().expect("First elf invalid start!");

    start = end + 1;
    end = start;
    while bytes[end] != b',' {
        end += 1;
    }
    let first_end = line[start..end].parse().expect("First elf invalid end!");

    start = end + 1;
    end = start;
    while bytes[end] != b'-' {
        end += 1;
    }
    let second_start = line[start..end].parse().expect("Second elf invalid start!");
    let second_end = line[end + 1..].parse().expect("Second elf invalid end!");

    // This is actually wasted effort. I was hoping there was some `range.contains(&range)` logic
    // but there doesn't appear to be. I'll leave this in here though because it's nice to use
    // random parts of Rust sometimes. Grease those wheels. Don't get ... rusty ...
    (first_start..=first_end, second_start..=second_end)
}

fn is_superset(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r1.start() <= r2.start() && r1.end() >= r2.end()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| {
            // TODO could optimize - if first_elf_start > second_elf_end then no need to parse the
            // other values.
            let (first_elf_range, second_elf_range) = elf_ranges(line);
            intersect(&first_elf_range, &second_elf_range)
        })
        .count() as u64
}

fn intersect(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    let disjoint = r1.end() < r2.start() || r1.start() > r2.end();
    !disjoint
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8\n\
                           2-3,4-5\n\
                           5-7,7-9\n\
                           2-8,3-7\n\
                           6-6,4-6\n\
                           2-6,4-8\n";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1(EXAMPLE), 2);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 584);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part2(EXAMPLE), 4);
        }

        #[test]
        fn my_input() {
            assert_eq!(part2(INPUT), 933);
        }
    }
}
