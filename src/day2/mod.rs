use std::cmp::Ordering;

use hand::RockPaperScissors;

mod hand;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');

            let opponent = parts.next().expect("Missing opponent");
            let us = parts.next().expect("Missing us");

            let opponent: RockPaperScissors = opponent.parse().unwrap();
            let us: RockPaperScissors = part1_mapping(us).parse().unwrap();

            us.play_against(&opponent)
        })
        .sum()
}

// In part 1 we thought "X" meant Rock which is "A", etc.
fn part1_mapping(s: &str) -> &str {
    match s {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => panic!("Invalid 'us' hand of {s:?}"),
    }
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');

            let opponent = parts.next().expect("Missing opponent");
            let us = parts.next().expect("Missing us");

            let opponent: RockPaperScissors = opponent.parse().unwrap();

            let needed_result = part2_mapping(us);
            let us = opponent.generate_hand(needed_result);

            us.play_against(&opponent)
        })
        .sum()
}

// In part 2 we know that "X" means we need to lose (e.g. `Ordering::Less`)
fn part2_mapping(s: &str) -> Ordering {
    match s {
        "X" => Ordering::Less,
        "Y" => Ordering::Equal,
        "Z" => Ordering::Greater,
        _ => panic!("Invalid 'match result' hand of {s:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let input = "A Y\nB X\nC Z\n";

            assert_eq!(part1(input), 15);
        }

        #[test]
        fn my_input() {
            let input = include_str!("./input.txt");

            assert_eq!(part1(input), 13009);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let input = "A Y\nB X\nC Z\n";

            assert_eq!(part2(input), 12);
        }

        #[test]
        fn my_input() {
            let input = include_str!("./input.txt");

            assert_eq!(part2(input), 10398);
        }
    }
}
