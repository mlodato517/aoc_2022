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
    let mut split = line.split([',', '-']);

    let first_start = split.next().expect("First elf start missing!");
    let first_end = split.next().expect("First elf end missing!");
    let second_start = split.next().expect("Second elf start missing!");
    let second_end = split.next().expect("Second elf end missing!");

    let first_start = first_start.parse().expect("First elf invalid start!");
    let first_end = first_end.parse().expect("First elf invalid end!");
    let second_start = second_start.parse().expect("Second elf invalid start!");
    let second_end = second_end.parse().expect("Second elf invalid end!");

    // This is actually wasted effort. I was hoping there was some `range.contains(&range)` logic
    // but there doesn't appear to be. I'll leave this in here though because it's nice to use
    // random parts of Rust sometimes. Grease those wheels. Don't get ... rusty ...
    (first_start..=first_end, second_start..=second_end)
}

fn is_superset(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r1.start() <= r2.start() && r1.end() >= r2.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let file = "2-4,6-8\n\
                    2-3,4-5\n\
                    5-7,7-9\n\
                    2-8,3-7\n\
                    6-6,4-6\n\
                    2-6,4-8\n";

            assert_eq!(part1(file), 2);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");

            assert_eq!(part1(file), 584);
        }
    }
}
