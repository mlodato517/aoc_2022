pub fn part1(input: &str) -> String {
    let mut stacks = Vec::new();

    let mut lines = input.lines();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        // Each box is a '[', a letter, and a ']' separated by spaces. The last stack doesn't have
        // a trailing space so we add 1 before dividing. This should only allocate once.
        let num_entries = (line.len() + 1) / 4;
        stacks.resize_with(num_entries, Vec::new);

        for (chunk, stack) in line.as_bytes().chunks(4).zip(stacks.iter_mut()) {
            if let [b'[', letter, ..] = chunk {
                stack.push(*letter);
            }
        }
    }

    // Our stacks are all upside-down. I think reversing them might be clearer than `VecDeque`.
    stacks.iter_mut().for_each(|stack| stack.reverse());

    for instruction in lines {
        let mut instructions = instruction.split(' ');

        let _move = instructions.next();
        let num_to_move = instructions.next().expect("Should have number of crates");
        let _from = instructions.next();
        let start_stack = instructions.next().expect("Should have starting stack");
        let _to = instructions.next();
        let end_stack = instructions.next().expect("Should have ending stack");

        let num_to_move = num_to_move
            .parse::<usize>()
            .expect("Number of crates should be a number");

        let start_stack = start_stack
            .parse::<usize>()
            .expect("Starting stack should be a number");
        let end_stack = end_stack
            .parse::<usize>()
            .expect("Ending stack should be a number");

        // The instructions are 1 indexed.
        let start_stack = start_stack - 1;
        let end_stack = end_stack - 1;

        for _ in 0..num_to_move {
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

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
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

            assert_eq!(part1(&file), "CMZ");
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part1(file), "ZWHVFWQWW");
        }
    }
}
