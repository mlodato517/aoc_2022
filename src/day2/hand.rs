use std::cmp::{Ordering, PartialOrd};
use std::str::FromStr;

#[derive(PartialEq, Eq)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}
impl RockPaperScissors {
    pub fn play_against(&self, other: &Self) -> u64 {
        let result_score = match self.cmp(other) {
            Ordering::Greater => 6,
            Ordering::Equal => 3,
            Ordering::Less => 0,
        };
        result_score + self.score()
    }

    pub fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}
impl PartialOrd for RockPaperScissors {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RockPaperScissors {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => Ordering::Less,
            (one, two) if one == two => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
}
impl FromStr for RockPaperScissors {
    type Err = std::convert::Infallible; // I'm lazy today

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().expect("Hand shouldn't be empty") {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => panic!("Invalid rock paper scissors hand of {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let opponent: RockPaperScissors = "A".parse().unwrap();
        let us: RockPaperScissors = "Y".parse().unwrap();

        assert_eq!(us.play_against(&opponent), 8);
    }

    #[test]
    fn example2() {
        let opponent: RockPaperScissors = "B".parse().unwrap();
        let us: RockPaperScissors = "X".parse().unwrap();

        assert_eq!(us.play_against(&opponent), 1);
    }

    #[test]
    fn example3() {
        let opponent: RockPaperScissors = "C".parse().unwrap();
        let us: RockPaperScissors = "Z".parse().unwrap();

        assert_eq!(us.play_against(&opponent), 6);
    }
}
