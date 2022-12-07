pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();

            let mut scratchpad = PriorityMask::default();

            // TODO: The problem makes it sounds like this is always even but we should check.
            let half = line.len() / 2;
            for byte in line.iter().take(half) {
                scratchpad.record_item(&PriorityMask::from(*byte));
            }

            line.iter()
                .skip(half)
                .map(|byte| PriorityMask::from(*byte))
                .find(|priority| scratchpad.contains(priority))
                .unwrap_or_else(|| panic!("No duplicate found in {line:?}"))
                .0
        })
        .sum()
}

#[derive(Clone, Copy, Default)]
struct PriorityMask(u64);
impl From<u8> for PriorityMask {
    fn from(b: u8) -> Self {
        // This only covers the bytes we expect from this problem
        let priority = if b <= b'Z' {
            b - b'A' + 27
        } else {
            b - b'a' + 1
        };
        Self(priority as u64)
    }
}
impl PriorityMask {
    fn record_item(&mut self, other: &Self) {
        self.0 |= 1 << other.0;
    }

    fn contains(&self, other: &Self) -> bool {
        self.0 & 1 << other.0 > 0
    }

    fn badge_from<I>(priorities: I) -> Option<u64>
    where
        I: IntoIterator<Item = Self>,
    {
        let mut single_item = priorities
            .into_iter()
            .map(|priority| priority.0)
            .fold(!0, |acc, priority| acc & priority);

        if single_item == 0 || !single_item.is_power_of_two() {
            return None;
        }

        // Waiting on `u64::ilog2`...
        let mut priority = 0;
        while single_item > 0 {
            single_item >>= 1;
            priority += 1;
        }

        // Subtract by 1 because priorites are 1..52 instead of 0..51
        Some(priority - 1)
    }
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut priority_sum = 0;

    while let Some(line) = lines.next() {
        let mut rucksacks = [PriorityMask::default(); 3];
        for byte in line.as_bytes() {
            let priority = PriorityMask::from(*byte);
            rucksacks[0].record_item(&priority);
        }

        // Get the two other elves in this group (we'll _assume_ :warning: they're there)
        // and remove their items from the list until there's only one left.
        for rucksack in rucksacks.iter_mut().skip(1) {
            let line = lines.next().expect("Elves should be in groups of three!");
            for byte in line.as_bytes() {
                let priority = PriorityMask::from(*byte);
                rucksack.record_item(&priority);
            }
        }

        priority_sum += PriorityMask::badge_from(rucksacks).unwrap();
    }

    priority_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let file = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                    PmmdzqPrVvPwwTWBwg\n\
                    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                    ttgJtRGJQctTZtZT\n\
                    CrZsJsPPZsGzwwsLwLmpwMDw";
            assert_eq!(part1(file), 157);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part1(file), 7863);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let file = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                    PmmdzqPrVvPwwTWBwg\n\
                    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                    ttgJtRGJQctTZtZT\n\
                    CrZsJsPPZsGzwwsLwLmpwMDw";
            assert_eq!(part2(file), 70);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part2(file), 2488);
        }
    }
}
