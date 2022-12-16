use std::str::FromStr;

pub fn part1(input: &str) -> String {
    let mut stacks = Vec::new();

    let mut lines = input.lines();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        parse_inverted_stacks(&mut stacks, line);
    }

    // Our stacks are all upside-down. I think reversing them might be clearer than `VecDeque`.
    stacks.iter_mut().for_each(|stack| stack.reverse());

    for instruction in lines {
        let Instruction {
            num_crates,
            start_stack,
            end_stack,
        } = instruction.parse().unwrap();

        // The instructions are 1 indexed.
        let start_stack = start_stack - 1;
        let end_stack = end_stack - 1;

        for _ in 0..num_crates {
            let c = stacks[start_stack]
                .pop()
                .expect("Asked to remove from empty stack");
            stacks[end_stack].push(c);
        }
    }

    let bytes: Vec<u8> = stacks
        .iter()
        .filter_map(|stack| stack.last().copied())
        .collect();
    String::from_utf8(bytes).expect("Each crate is a valid ASCII letter")
}

fn parse_inverted_stacks(stacks: &mut Vec<Vec<u8>>, line: &str) {
    // Each box is a '[', a letter, and a ']' separated by spaces. The last stack doesn't have
    // a trailing space so we add 1 before dividing. This should only resize once.
    let num_entries = (line.len() + 1) / 4;
    stacks.resize_with(num_entries, Vec::new);

    for (chunk, stack) in line.as_bytes().chunks(4).zip(stacks.iter_mut()) {
        if let [b'[', letter, ..] = chunk {
            stack.push(*letter);
        }
    }
}

struct Instruction {
    num_crates: usize,

    // 1 indexed
    start_stack: usize,

    // 1 indexed
    end_stack: usize,
}

impl FromStr for Instruction {
    type Err = std::convert::Infallible; // _still_ lazy

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let mut instructions = instruction.split(' ');

        let _move = instructions.next();
        let num_crates = instructions.next().expect("Should have number of crates");
        let _from = instructions.next();
        let start_stack = instructions.next().expect("Should have starting stack");
        let _to = instructions.next();
        let end_stack = instructions.next().expect("Should have ending stack");

        let num_crates = num_crates
            .parse::<usize>()
            .expect("Number of crates should be a number");

        let start_stack = start_stack
            .parse::<usize>()
            .expect("Starting stack should be a number");
        let end_stack = end_stack
            .parse::<usize>()
            .expect("Ending stack should be a number");

        Ok(Self {
            num_crates,
            start_stack,
            end_stack,
        })
    }
}

pub fn part2(input: &str) -> String {
    let mut stacks = Vec::new();

    let mut lines = input.lines();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        parse_inverted_stacks(&mut stacks, line);
    }

    // Our stacks are all upside-down. I think reversing them might be clearer than `VecDeque`.
    stacks.iter_mut().for_each(|stack| stack.reverse());

    for instruction in lines {
        let Instruction {
            num_crates,
            start_stack,
            end_stack,
        } = instruction.parse().unwrap();

        // The instructions are 1 indexed.
        let start_stack = start_stack - 1;
        let end_stack = end_stack - 1;

        //  Move 1 from 3 to 5 (1 indexed)
        //  Move 1 from 2 to 4 (0 indexed)
        //      split at 2 + 1 = 3
        //      (start)    (end)
        //      [0, 1, 2], [3, 4, 5, 6, 7, 8]
        //      start[start_index] = start[2], end[end_index - start_index - 1] = end[1]
        //
        //  Move 1 from 5 to 3 (1 indexed)
        //  Move 1 from 4 to 2 (0 indexed)
        //      split at 2 + 1 = 3
        //      (end)      (start)
        //      [0, 1, 2], [3, 4, 5, 6, 7, 8]
        //      (start)             (end)
        //      [3, 4, 5, 6, 7, 8], [0, 1, 2]
        //      start[start_index - end_index - 1] = start[1], end[end_index] = end[2]
        //

        // Waiting on https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.get_many_mut
        let (start, end) = if start_stack < end_stack {
            let (contains_start, contains_end) = stacks.split_at_mut(start_stack + 1);
            (
                &mut contains_start[start_stack],
                &mut contains_end[end_stack - start_stack - 1],
            )
        } else {
            let (contains_end, contains_start) = stacks.split_at_mut(end_stack + 1);
            (
                &mut contains_start[start_stack - end_stack - 1],
                &mut contains_end[end_stack],
            )
        };
        end.extend(start.drain(start.len() - num_crates..));
    }

    let bytes: Vec<u8> = stacks
        .iter()
        .filter_map(|stack| stack.last().copied())
        .collect();
    String::from_utf8(bytes).expect("Each crate is a valid ASCII letter")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_file() -> String {
        // Note whitespace is important here!
        let mut file = "    [D]    \n".to_string();
        file.push_str("[N] [C]    \n");
        file.push_str("[Z] [M] [P]\n");
        file.push_str(" 1   2   3 \n");
        file.push('\n');
        file.push_str("move 1 from 2 to 1\n");
        file.push_str("move 3 from 1 to 3\n");
        file.push_str("move 2 from 2 to 1\n");
        file.push_str("move 1 from 1 to 2\n");

        file
    }

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1(&example_file()), "CMZ");
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), "ZWHVFWQWW");
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part2(&example_file()), "MCD");
        }

        #[test]
        fn my_input() {
            assert_eq!(part2(INPUT), "HZFZCCWWV");
        }
    }
}
