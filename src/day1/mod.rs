pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .scan(0, |calorie_count, line| {
            *calorie_count = new_calories(*calorie_count, line);
            Some(*calorie_count)
        })
        .max()
        .unwrap_or_default()
}

fn new_calories(current_calories: u64, line: &str) -> u64 {
    if line.is_empty() {
        0
    } else {
        let calories: u64 = line.parse().expect("Calories must be a number");
        current_calories + calories
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_elf_one_line() {
        let file = "1000\n";
        let max_calories = part1(file);

        assert_eq!(max_calories, 1000);
    }

    #[test]
    fn one_elf_multiple_lines() {
        let file = "1000\n1000\n";
        let max_calories = part1(file);

        assert_eq!(max_calories, 2000);
    }

    #[test]
    fn two_elves_one_line_max_first() {
        let file = "2000\n\n1000\n";
        let max_calories = part1(file);

        assert_eq!(max_calories, 2000);
    }

    #[test]
    fn two_elves_one_line_max_last() {
        let file = "1000\n\n2000\n";
        let max_calories = part1(file);

        assert_eq!(max_calories, 2000);
    }

    #[test]
    fn two_elves_multiple_lines_max_first() {
        let file = "1000\n1000\n\n1000\n";
        let max_calories = part1(file);

        assert_eq!(max_calories, 2000);
    }

    #[test]
    fn two_elves_multiple_lines_max_last() {
        let file = "1000\n\n1000\n1000\n";
        let max_calories = part1(file);

        assert_eq!(max_calories, 2000);
    }
}
