use aoc_2022::day1;
use aoc_2022::day10;
use aoc_2022::day11;
use aoc_2022::day12;
use aoc_2022::day13;
use aoc_2022::day2;
use aoc_2022::day3;
use aoc_2022::day4;
use aoc_2022::day5;
use aoc_2022::day6;
use aoc_2022::day7;
use aoc_2022::day8;
use aoc_2022::day9;

const DAY_1_INPUT: &str = include_str!("./day1/input.txt");
const DAY_2_INPUT: &str = include_str!("./day2/input.txt");
const DAY_3_INPUT: &str = include_str!("./day3/input.txt");
const DAY_4_INPUT: &str = include_str!("./day4/input.txt");
const DAY_5_INPUT: &str = include_str!("./day5/input.txt");
const DAY_6_INPUT: &str = include_str!("./day6/input.txt");
const DAY_7_INPUT: &str = include_str!("./day7/input.txt");
const DAY_8_INPUT: &str = include_str!("./day8/input.txt");
const DAY_9_INPUT: &str = include_str!("./day9/input.txt");
const DAY_10_INPUT: &str = include_str!("./day10/input.txt");
const DAY_11_INPUT: &str = include_str!("./day11/input.txt");
const DAY_12_INPUT: &str = include_str!("./day12/input.txt");
const DAY_13_INPUT: &str = include_str!("./day13/input.txt");

fn main() {
    println!("Day 1 Part 1 - {}", day1::part1(DAY_1_INPUT));
    println!("Day 1 Part 2 - {}", day1::part2(DAY_1_INPUT));

    println!("Day 2 Part 1 - {}", day2::part1(DAY_2_INPUT));
    println!("Day 2 Part 2 - {}", day2::part2(DAY_2_INPUT));

    println!("Day 3 Part 1 - {}", day3::part1(DAY_3_INPUT));
    println!("Day 3 Part 2 - {}", day3::part2(DAY_3_INPUT));

    println!("Day 4 Part 1 - {}", day4::part1(DAY_4_INPUT));
    println!("Day 4 Part 2 - {}", day4::part2(DAY_4_INPUT));

    println!("Day 5 Part 1 - {}", day5::part1(DAY_5_INPUT));
    println!("Day 5 Part 2 - {}", day5::part2(DAY_5_INPUT));

    println!("Day 6 Part 1 - {}", day6::part1(DAY_6_INPUT));
    println!("Day 6 Part 2 - {}", day6::part2(DAY_6_INPUT));

    println!("Day 7 Part 1 - {}", day7::part1(DAY_7_INPUT));
    println!("Day 7 Part 2 - {}", day7::part2(DAY_7_INPUT));

    println!("Day 8 Part 1 - {}", day8::part1(DAY_8_INPUT));
    println!("Day 8 Part 2 - {}", day8::part2(DAY_8_INPUT));

    println!("Day 9 Part 1 - {}", day9::part1(DAY_9_INPUT));
    println!("Day 9 Part 2 - {}", day9::part2(DAY_9_INPUT));

    println!("Day 10 Part 1 - {}", day10::part1(DAY_10_INPUT));
    println!("Day 10 Part 2");
    let day10_part2 = day10::part2(DAY_10_INPUT);
    for row in day10_part2 {
        println!("\t{}", std::str::from_utf8(&row).unwrap());
    }

    println!("Day 11 Part 1 - {}", day11::part1(DAY_11_INPUT));
    println!("Day 11 Part 2 - {}", day11::part2(DAY_11_INPUT));

    println!("Day 12 Part 1 - {}", day12::part1(DAY_12_INPUT));
    println!("Day 12 Part 2 - {}", day12::part2(DAY_12_INPUT));

    println!("Day 13 Part 1 - {}", day13::part1(DAY_13_INPUT));
    println!("Day 13 Part 2 - {}", day13::part2(DAY_13_INPUT));
}
