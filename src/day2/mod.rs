use std::cmp::Ordering;

use hand::RockPaperScissors;

mod hand;

pub fn part1(input: &str) -> u64 {
    input
        .as_bytes()
        .chunks_exact(4)
        .map(|line| {
            let opponent = line[0];
            let us = line[2];

            let opponent = RockPaperScissors::from(opponent);
            let us = RockPaperScissors::from(part1_mapping(us));

            us.play_against(&opponent)
        })
        .sum()
}

// In part 1 we thought "X" meant Rock which is "A", etc.
fn part1_mapping(b: u8) -> u8 {
    match b {
        b'X' => b'A',
        b'Y' => b'B',
        _ => b'C',
    }
}

pub fn part2(input: &str) -> u64 {
    input
        .as_bytes()
        .chunks_exact(4)
        .map(|line| {
            let opponent = line[0];
            let us = line[2];

            let opponent = RockPaperScissors::from(opponent);

            let needed_result = part2_mapping(us);
            let us = opponent.generate_hand(needed_result);

            us.play_against(&opponent)
        })
        .sum()
}

// In part 2 we know that "X" means we need to lose (e.g. `Ordering::Less`)
fn part2_mapping(b: u8) -> Ordering {
    match b {
        b'X' => Ordering::Less,
        b'Y' => Ordering::Equal,
        _ => Ordering::Greater,
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
