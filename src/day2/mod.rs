use hand::RockPaperScissors;

mod hand;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');

            let opponent = parts.next().expect("Missing opponent");
            let us = parts.next().expect("Missing us");

            let us: RockPaperScissors = us.parse().unwrap();
            let opponent = opponent.parse().unwrap();

            us.play_against(&opponent)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "A Y\nB X\nC Z\n";

        assert_eq!(part1(input), 15);
    }
}
