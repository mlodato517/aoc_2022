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

    mod part1 {
        use super::*;

        #[test]
        fn examples() {
            let file = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
            assert_eq!(part1(file), 7);

            let file = "bvwbjplbgvbhsrlpgdmjqwftvncz";
            assert_eq!(part1(file), 5);

            let file = "nppdvjthqldpwncqszvftbrmjlhg";
            assert_eq!(part1(file), 6);

            let file = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
            assert_eq!(part1(file), 10);

            let file = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
            assert_eq!(part1(file), 11);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part1(file), 1723);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn examples() {
            let file = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
            assert_eq!(part2(file), 19);

            let file = "bvwbjplbgvbhsrlpgdmjqwftvncz";
            assert_eq!(part2(file), 23);

            let file = "nppdvjthqldpwncqszvftbrmjlhg";
            assert_eq!(part2(file), 23);

            let file = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
            assert_eq!(part2(file), 29);

            let file = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
            assert_eq!(part2(file), 26);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part2(file), 3708);
        }
    }
}
