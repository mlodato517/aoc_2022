pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .scan(0, |calorie_count, line| {
            let new_calories = if line.is_empty() {
                0
            } else {
                let calories: u64 = line.parse().expect("Calories must be a number");
                *calorie_count + calories
            };
            *calorie_count = new_calories;
            Some(*calorie_count)
        })
        .max()
        .unwrap_or_default()
}

pub fn part2(input: &str) -> u64 {
    // Could use an array but this keeps the sort order clearer.
    let mut smallest = 0;
    let mut middle = 0;
    let mut biggest = 0;

    let mut elf_calories = 0;

    for line in input.lines() {
        if line.is_empty() {
            if elf_calories > biggest {
                smallest = middle;
                middle = biggest;
                biggest = elf_calories;
            } else if elf_calories > middle {
                smallest = middle;
                middle = elf_calories;
            } else if elf_calories > smallest {
                smallest = elf_calories;
            }
            elf_calories = 0;
        } else {
            let calories: u64 = line.parse().expect("Calories must be a number");
            elf_calories += calories;
        }
    }
    if elf_calories > biggest {
        smallest = middle;
        middle = biggest;
        biggest = elf_calories;
    } else if elf_calories > middle {
        smallest = middle;
        middle = elf_calories;
    } else if elf_calories > smallest {
        smallest = elf_calories;
    };

    smallest + middle + biggest
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
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

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            let max_calories = part1(file);

            assert_eq!(max_calories, 67027);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn one_elf_one_line() {
            let file = "1000\n";
            let sum_max_3 = part2(file);

            assert_eq!(sum_max_3, 1000);
        }

        #[test]
        fn one_elf_multiple_lines() {
            let file = "1000\n1000\n";
            let sum_max_3 = part2(file);

            assert_eq!(sum_max_3, 2000);
        }

        #[test]
        fn two_elves_one_line() {
            let file = "2000\n\n1000\n";
            let sum_max_3 = part2(file);

            assert_eq!(sum_max_3, 3000);
        }

        #[test]
        fn two_elves_two_lines() {
            let file = "1000\n1000\n\n1000\n1000\n";
            let sum_max_3 = part2(file);

            assert_eq!(sum_max_3, 4000);
        }

        #[test]
        fn four_elves_max_013() {
            let file = "1000\n\n2000\n\n500\n\n3000";
            let sum_max_3 = part2(file);

            assert_eq!(sum_max_3, 6000);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            let max_calories = part2(file);

            assert_eq!(max_calories, 197291);
        }
    }
}
