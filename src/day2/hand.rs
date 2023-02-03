use std::cmp::Ordering;

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
            // Hopefully the compiler eliminates this addition since `.into()` subtracts it.
            n => (n as u8 + b'A').into(),
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

impl From<u8> for RockPaperScissors {
    fn from(value: u8) -> Self {
        match value - b'A' {
            0 => Self::Rock,
            1 => Self::Paper,
            _ => Self::Scissors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let opponent: RockPaperScissors = b'A'.into();
        let us: RockPaperScissors = b'B'.into();

        assert_eq!(us.play_against(&opponent), 8);
    }

    #[test]
    fn example2() {
        let opponent: RockPaperScissors = b'B'.into();
        let us: RockPaperScissors = b'A'.into();

        assert_eq!(us.play_against(&opponent), 1);
    }

    #[test]
    fn example3() {
        let opponent: RockPaperScissors = b'C'.into();
        let us: RockPaperScissors = b'C'.into();

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
