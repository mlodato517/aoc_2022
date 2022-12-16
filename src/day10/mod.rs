pub fn part1(input: &str) -> i64 {
    let mut signal_strength_sum = 0;

    let during = |current_cycle, x_register| {
        if [20, 60, 100, 140, 180, 220].contains(&current_cycle) {
            signal_strength_sum += current_cycle as i64 * x_register;
        }
    };

    execute_instructions(input, during);

    signal_strength_sum
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn latency(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }

    fn update_register(self, register: &mut i64) {
        match self {
            Self::Noop => {}
            Self::Addx(value) => *register += value,
        }
    }
}

fn parse_instruction(instruction: &str) -> Instruction {
    let mut split = instruction.split(' ');

    let instruction_type = split
        .next()
        .expect("Every instruction should be noop or addx");

    match instruction_type {
        "noop" => Instruction::Noop,
        "addx" => {
            let value = split.next().expect("addx instruction should have a value");
            let value = value.parse().expect("addx value was an invalid integer");
            Instruction::Addx(value)
        }
        _ => panic!("Invalid instruction - {instruction:?}"),
    }
}

const HEIGHT: usize = 6;
const WIDTH: usize = 40;
pub fn part2(input: &str) -> [[u8; WIDTH]; HEIGHT] {
    let mut screen = [[b'.'; WIDTH]; 6];

    let during = |current_cycle: usize, sprite_mid_position: i64| {
        let position = current_cycle - 1;
        let row = position / WIDTH;
        let col = position % WIDTH;
        if sprite_mid_position.abs_diff(col as i64) <= 1 {
            screen[row][col] = b'#';
        }
    };

    execute_instructions(input, during);

    screen
}

fn execute_instructions<F>(input: &str, mut during: F)
where
    F: FnMut(usize, i64),
{
    let mut x_register = 1;

    let mut current_cycle = 0;

    for input in input.lines() {
        // BEGIN START
        // This is the start of a cycle. We'll read instructions and begin executing them.
        current_cycle += 1;

        let instruction = parse_instruction(input);
        let target_cycle = current_cycle + instruction.latency() - 1;
        // END START

        loop {
            // BEGIN DURING
            // This is during a cycle. The register can be read.
            during(current_cycle, x_register);
            // END DURING

            // BEGIN AFTER
            // This is after a cycle. The register is updated if the instruction has completed.
            if current_cycle == target_cycle {
                instruction.update_register(&mut x_register);
                break;
            } else {
                current_cycle += 1;
            }
            // END AFTER

            // Another BEGIN/END START here but we aren't going to read/begin another instruction
            // until we're done with the current one.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let file = "addx 15\n\
                        addx -11\n\
                        addx 6\n\
                        addx -3\n\
                        addx 5\n\
                        addx -1\n\
                        addx -8\n\
                        addx 13\n\
                        addx 4\n\
                        noop\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx -35\n\
                        addx 1\n\
                        addx 24\n\
                        addx -19\n\
                        addx 1\n\
                        addx 16\n\
                        addx -11\n\
                        noop\n\
                        noop\n\
                        addx 21\n\
                        addx -15\n\
                        noop\n\
                        noop\n\
                        addx -3\n\
                        addx 9\n\
                        addx 1\n\
                        addx -3\n\
                        addx 8\n\
                        addx 1\n\
                        addx 5\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx -36\n\
                        noop\n\
                        addx 1\n\
                        addx 7\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 2\n\
                        addx 6\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 1\n\
                        noop\n\
                        noop\n\
                        addx 7\n\
                        addx 1\n\
                        noop\n\
                        addx -13\n\
                        addx 13\n\
                        addx 7\n\
                        noop\n\
                        addx 1\n\
                        addx -33\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 2\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 8\n\
                        noop\n\
                        addx -1\n\
                        addx 2\n\
                        addx 1\n\
                        noop\n\
                        addx 17\n\
                        addx -9\n\
                        addx 1\n\
                        addx 1\n\
                        addx -3\n\
                        addx 11\n\
                        noop\n\
                        noop\n\
                        addx 1\n\
                        noop\n\
                        addx 1\n\
                        noop\n\
                        noop\n\
                        addx -13\n\
                        addx -19\n\
                        addx 1\n\
                        addx 3\n\
                        addx 26\n\
                        addx -30\n\
                        addx 12\n\
                        addx -1\n\
                        addx 3\n\
                        addx 1\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx -9\n\
                        addx 18\n\
                        addx 1\n\
                        addx 2\n\
                        noop\n\
                        noop\n\
                        addx 9\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx -1\n\
                        addx 2\n\
                        addx -37\n\
                        addx 1\n\
                        addx 3\n\
                        noop\n\
                        addx 15\n\
                        addx -21\n\
                        addx 22\n\
                        addx -6\n\
                        addx 1\n\
                        noop\n\
                        addx 2\n\
                        addx 1\n\
                        noop\n\
                        addx -10\n\
                        noop\n\
                        noop\n\
                        addx 20\n\
                        addx 1\n\
                        addx 2\n\
                        addx 2\n\
                        addx -6\n\
                        addx -11\n\
                        noop\n\
                        noop\n\
                        noop";
            assert_eq!(part1(file), 13140);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part1(file), 13720);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let file = "addx 15\n\
                        addx -11\n\
                        addx 6\n\
                        addx -3\n\
                        addx 5\n\
                        addx -1\n\
                        addx -8\n\
                        addx 13\n\
                        addx 4\n\
                        noop\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx 5\n\
                        addx -1\n\
                        addx -35\n\
                        addx 1\n\
                        addx 24\n\
                        addx -19\n\
                        addx 1\n\
                        addx 16\n\
                        addx -11\n\
                        noop\n\
                        noop\n\
                        addx 21\n\
                        addx -15\n\
                        noop\n\
                        noop\n\
                        addx -3\n\
                        addx 9\n\
                        addx 1\n\
                        addx -3\n\
                        addx 8\n\
                        addx 1\n\
                        addx 5\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx -36\n\
                        noop\n\
                        addx 1\n\
                        addx 7\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 2\n\
                        addx 6\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 1\n\
                        noop\n\
                        noop\n\
                        addx 7\n\
                        addx 1\n\
                        noop\n\
                        addx -13\n\
                        addx 13\n\
                        addx 7\n\
                        noop\n\
                        addx 1\n\
                        addx -33\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 2\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx 8\n\
                        noop\n\
                        addx -1\n\
                        addx 2\n\
                        addx 1\n\
                        noop\n\
                        addx 17\n\
                        addx -9\n\
                        addx 1\n\
                        addx 1\n\
                        addx -3\n\
                        addx 11\n\
                        noop\n\
                        noop\n\
                        addx 1\n\
                        noop\n\
                        addx 1\n\
                        noop\n\
                        noop\n\
                        addx -13\n\
                        addx -19\n\
                        addx 1\n\
                        addx 3\n\
                        addx 26\n\
                        addx -30\n\
                        addx 12\n\
                        addx -1\n\
                        addx 3\n\
                        addx 1\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx -9\n\
                        addx 18\n\
                        addx 1\n\
                        addx 2\n\
                        noop\n\
                        noop\n\
                        addx 9\n\
                        noop\n\
                        noop\n\
                        noop\n\
                        addx -1\n\
                        addx 2\n\
                        addx -37\n\
                        addx 1\n\
                        addx 3\n\
                        noop\n\
                        addx 15\n\
                        addx -21\n\
                        addx 22\n\
                        addx -6\n\
                        addx 1\n\
                        noop\n\
                        addx 2\n\
                        addx 1\n\
                        noop\n\
                        addx -10\n\
                        noop\n\
                        noop\n\
                        addx 20\n\
                        addx 1\n\
                        addx 2\n\
                        addx 2\n\
                        addx -6\n\
                        addx -11\n\
                        noop\n\
                        noop\n\
                        noop";
            part2(file);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            let part2_output = part2(file);
            let part2_borrowed = [
                &part2_output[0],
                &part2_output[1],
                &part2_output[2],
                &part2_output[3],
                &part2_output[4],
                &part2_output[5],
            ];

            let expected = [
                b"####.###..#..#.###..#..#.####..##..#..#.",
                b"#....#..#.#..#.#..#.#..#....#.#..#.#..#.",
                b"###..###..#..#.#..#.####...#..#....####.",
                b"#....#..#.#..#.###..#..#..#...#....#..#.",
                b"#....#..#.#..#.#.#..#..#.#....#..#.#..#.",
                b"#....###...##..#..#.#..#.####..##..#..#.",
            ];

            assert_eq!(part2_borrowed, expected);
        }
    }
}
