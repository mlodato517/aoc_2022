use aoc_2022::day1;
use aoc_2022::day2;

const DAY_1_INPUT: &str = include_str!("./day1/input.txt");
const DAY_2_INPUT: &str = include_str!("./day2/input.txt");

fn main() {
    println!("Day 1 Part 1 - {}", day1::part1(DAY_1_INPUT));
    println!("Day 1 Part 2 - {}", day1::part2(DAY_1_INPUT));

    println!("Day 2 Part 1 - {}", day2::part1(DAY_2_INPUT));
    println!("Day 2 Part 2 - {}", day2::part2(DAY_2_INPUT));
}