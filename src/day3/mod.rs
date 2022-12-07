pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();

            // Godbolt shows this as equivalent to putting this outside the loop. This could also
            // allow easier parallelization.
            let mut scratchpad = [false; 52];

            // TODO: The problem makes it sounds like this is always even but we should check.
            let half = line.len() / 2;
            for byte in line.iter().take(half) {
                let idx = byte_to_priority(*byte);

                // Avoid bounds check here and below?
                scratchpad[idx as usize - 1] = true;
            }

            line.iter()
                .skip(half)
                .map(|byte| byte_to_priority(*byte))
                .find(|priority| scratchpad[*priority as usize - 1])
                .unwrap_or_else(|| panic!("No duplicate found in {line:?}"))
        })
        .sum()
}

fn byte_to_priority(b: u8) -> u64 {
    let priority = if b <= b'Z' {
        b - b'A' + 27
    } else {
        b - b'a' + 1
    };
    priority as u64
}

#[cfg(test)]
mod tests {
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
