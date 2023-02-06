use std::fmt::Debug;
use std::time::{Duration, Instant};

use aoc_2022::day1;
use aoc_2022::day10;
use aoc_2022::day11;
use aoc_2022::day12;
use aoc_2022::day13;
use aoc_2022::day14;
use aoc_2022::day15;
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
const DAY_14_INPUT: &str = include_str!("./day14/input.txt");
const DAY_15_INPUT: &str = include_str!("./day15/input.txt");

fn main() {
    println!("Day 1 Part 1 - {:?}", with_timing(day1::part1, DAY_1_INPUT));
    println!("Day 1 Part 2 - {:?}", with_timing(day1::part2, DAY_1_INPUT));

    println!("Day 2 Part 1 - {:?}", with_timing(day2::part1, DAY_2_INPUT));
    println!("Day 2 Part 2 - {:?}", with_timing(day2::part2, DAY_2_INPUT));

    println!("Day 3 Part 1 - {:?}", with_timing(day3::part1, DAY_3_INPUT));
    println!("Day 3 Part 2 - {:?}", with_timing(day3::part2, DAY_3_INPUT));

    println!("Day 4 Part 1 - {:?}", with_timing(day4::part1, DAY_4_INPUT));
    println!("Day 4 Part 2 - {:?}", with_timing(day4::part2, DAY_4_INPUT));

    println!("Day 5 Part 1 - {:?}", with_timing(day5::part1, DAY_5_INPUT));
    println!("Day 5 Part 2 - {:?}", with_timing(day5::part2, DAY_5_INPUT));

    println!("Day 6 Part 1 - {:?}", with_timing(day6::part1, DAY_6_INPUT));
    println!("Day 6 Part 2 - {:?}", with_timing(day6::part2, DAY_6_INPUT));

    println!("Day 7 Part 1 - {:?}", with_timing(day7::part1, DAY_7_INPUT));
    println!("Day 7 Part 2 - {:?}", with_timing(day7::part2, DAY_7_INPUT));

    println!("Day 8 Part 1 - {:?}", with_timing(day8::part1, DAY_8_INPUT));
    println!("Day 8 Part 2 - {:?}", with_timing(day8::part2, DAY_8_INPUT));

    println!("Day 9 Part 1 - {:?}", with_timing(day9::part1, DAY_9_INPUT));
    println!("Day 9 Part 2 - {:?}", with_timing(day9::part2, DAY_9_INPUT));

    println!(
        "Day 10 Part 1 - {:?}",
        with_timing(day10::part1, DAY_10_INPUT)
    );

    let day10_part2 = with_timing(day10::part2, DAY_10_INPUT);
    println!("Day 10 Part 2 - {:?}", day10_part2.1);
    for row in day10_part2.0 {
        println!("\t{}", std::str::from_utf8(&row).unwrap());
    }

    println!(
        "Day 11 Part 1 - {:?}",
        with_timing(day11::part1, DAY_11_INPUT)
    );
    println!(
        "Day 11 Part 2 - {:?}",
        with_timing(day11::part2, DAY_11_INPUT)
    );

    println!(
        "Day 12 Part 1 - {:?}",
        with_timing(day12::part1, DAY_12_INPUT)
    );
    println!(
        "Day 12 Part 2 - {:?}",
        with_timing(day12::part2, DAY_12_INPUT)
    );

    println!(
        "Day 13 Part 1 - {:?}",
        with_timing(day13::part1, DAY_13_INPUT)
    );
    println!(
        "Day 13 Part 2 - {:?}",
        with_timing(day13::part2, DAY_13_INPUT)
    );

    println!(
        "Day 14 Part 1 - {:?}",
        with_timing(day14::part1, DAY_14_INPUT)
    );
    println!(
        "Day 14 Part 2 - {:?}",
        with_timing(day14::part2, DAY_14_INPUT)
    );

    println!(
        "Day 15 Part 1 - {:?}",
        with_timing(day15::part1, (DAY_15_INPUT, 2_000_000))
    );
    println!(
        "Day 15 Part 2 - {:?}",
        with_timing(day15::part2, (DAY_15_INPUT, 0..=4_000_000, 0..=4_000_000))
    );
}

fn with_timing<F, A, T>(f: F, input: A) -> (T, Duration)
where
    F: Fn(A) -> T + Send + 'static,
    T: Debug + Send + 'static,
    A: Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let start = Instant::now();
        let result = f(input);
        let time = start.elapsed();
        tx.send((result, time))
            .expect("Sender shouldn't have disconnected yet");
    });

    match rx.recv_timeout(std::time::Duration::from_secs(15)) {
        Ok((result, time)) => (result, time),
        Err(_) => panic!("Day failed to complete in 15s!"),
    }
}
