use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RockPaperScissors {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
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
        *self as u64 + 1
    }

    /// Generates a hand that should return the desired result when played against `&self`.
    /// For example, if we're `Rock` and we want `Ordering::Less` that means we want a losing hand
    /// against `Rock` so we want `Scissors`.
    pub fn generate_hand(&self, ordering: Ordering) -> Self {
        let value = *self as i8;
        let new_value = value + ordering as i8;

        // `value` was 0, 1, or 2 and is now -1, 0, 1, 2, or 3.
        //
        // -1 => we're Rock and we want a losing hand so we have to generate Scissors which is 2.
        // 3 => we're Scissors and we want a winning hand so we have to generate Rock which is 0.
        //
        // We can do this branchlessly by working mod 3:
        //
        //    let mapped_value = new_value.rem_euclid(3) as u8;
        //
        // But the resulting assembly is longer so it's not obvious which is faster. Will need to
        // benchmark and see if it vectorizes.
        match new_value {
            -1 => Self::Scissors,
            3 => Self::Rock,
            n => (n as u8).try_into().unwrap(),
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
            (Self::Scissors, Self::Rock) => Ordering::Less,
            (Self::Rock, Self::Scissors) => Ordering::Greater,
            _ => (*self as u8).cmp(&(*other as u8)),
        }
    }
}

impl FromStr for RockPaperScissors {
    type Err = std::convert::Infallible; // I'm lazy today

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => panic!("Invalid rock paper scissors hand of {s:?}"),
        }
    }
}

impl TryFrom<u8> for RockPaperScissors {
    type Error = std::convert::Infallible; // I'm still lazy

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Rock),
            1 => Ok(Self::Paper),
            2 => Ok(Self::Scissors),
            _ => panic!("Invalid rock paper scissors value of '{value}'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let opponent: RockPaperScissors = "A".parse().unwrap();
        let us: RockPaperScissors = "B".parse().unwrap();

        assert_eq!(us.play_against(&opponent), 8);
    }

    #[test]
    fn example2() {
        let opponent: RockPaperScissors = "B".parse().unwrap();
        let us: RockPaperScissors = "A".parse().unwrap();

        assert_eq!(us.play_against(&opponent), 1);
    }

    #[test]
    fn example3() {
        let opponent: RockPaperScissors = "C".parse().unwrap();
        let us: RockPaperScissors = "C".parse().unwrap();

        assert_eq!(us.play_against(&opponent), 6);
    }

    #[test]
    fn test_generate_hand() {
        assert_eq!(
            RockPaperScissors::Rock.generate_hand(Ordering::Less),
            RockPaperScissors::Scissors,
        );
        assert_eq!(
            RockPaperScissors::Rock.generate_hand(Ordering::Equal),
            RockPaperScissors::Rock,
        );
        assert_eq!(
            RockPaperScissors::Rock.generate_hand(Ordering::Greater),
            RockPaperScissors::Paper,
        );

        assert_eq!(
            RockPaperScissors::Paper.generate_hand(Ordering::Less),
            RockPaperScissors::Rock,
        );
        assert_eq!(
            RockPaperScissors::Paper.generate_hand(Ordering::Equal),
            RockPaperScissors::Paper,
        );
        assert_eq!(
            RockPaperScissors::Paper.generate_hand(Ordering::Greater),
            RockPaperScissors::Scissors,
        );

        assert_eq!(
            RockPaperScissors::Scissors.generate_hand(Ordering::Less),
            RockPaperScissors::Paper,
        );
        assert_eq!(
            RockPaperScissors::Scissors.generate_hand(Ordering::Equal),
            RockPaperScissors::Scissors,
        );
        assert_eq!(
            RockPaperScissors::Scissors.generate_hand(Ordering::Greater),
            RockPaperScissors::Rock,
        );
    }
}
