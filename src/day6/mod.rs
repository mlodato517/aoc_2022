pub fn part1(input: &str) -> usize {
    let window_length = 4;
    let start_of_first_unique_window = input
        .as_bytes()
        .windows(window_length)
        .position(|window| {
            let mut items = window
                .try_into()
                .expect("windows() should return the correct size slice");
            is_unique_short(&mut items)
        })
        .expect("Should be an answer!");
    start_of_first_unique_window + window_length
}

// Did some invalid benchtesting on:
// https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=92b9bab94615b6f3191db1386c2daee4
// and it seems like this is the way to go :shrug:
fn is_unique_short(items: &mut [u8; 4]) -> bool {
    items.sort_unstable();
    items.windows(2).all(|window| window[0] != window[1])
}

pub fn part2(input: &str) -> usize {
    let window_length = 14;
    let start_of_first_unique_window = input
        .as_bytes()
        .windows(window_length)
        .position(|window| {
            let mut items = window
                .try_into()
                .expect("windows() should return the correct size slice");
            is_unique_long(&mut items)
        })
        .expect("Should be an answer!");
    start_of_first_unique_window + window_length
}

// TODO should this be const generic? Should we switch to a HashSet (or similar) when the number
// gets large? For something like 14 items should we even use `.windows()` this way? I feel like
// it'd be better to maintain a single `HashSet` and pop off the first item and push on the last
// item each iteration. But I don't feel like adding a dependency yet... even though hashbrown is
// already in std. Wow I'm lazy.
fn is_unique_long(items: &mut [u8; 14]) -> bool {
    items.sort_unstable();
    items.windows(2).all(|window| window[0] != window[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn examples() {
            assert_eq!(part1(EXAMPLE1), 7);
            assert_eq!(part1(EXAMPLE2), 5);
            assert_eq!(part1(EXAMPLE3), 6);
            assert_eq!(part1(EXAMPLE4), 10);
            assert_eq!(part1(EXAMPLE5), 11);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 1723);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn examples() {
            assert_eq!(part2(EXAMPLE1), 19);
            assert_eq!(part2(EXAMPLE2), 23);
            assert_eq!(part2(EXAMPLE3), 23);
            assert_eq!(part2(EXAMPLE4), 29);
            assert_eq!(part2(EXAMPLE5), 26);
        }

        #[test]
        fn my_input() {
            assert_eq!(part2(INPUT), 3708);
        }
    }
}
