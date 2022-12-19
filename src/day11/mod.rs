mod monkey;

use monkey::Monkey;

pub fn part1(input: &str) -> u64 {
    let monkeys = parse_monkeys(input);
    let manage_worry = |worry| worry / 3;

    let mut inspection_count = keep_away_activity_count(monkeys, manage_worry, 20);

    // Probably faster to just run a "max" with two buckets but my input only has 8 monkeys.
    inspection_count.sort_unstable();
    inspection_count.into_iter().rev().take(2).product()
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(monkey::parse_block).collect()
}

pub fn part2(input: &str) -> u64 {
    let monkeys = parse_monkeys(input);
    let modulus: i64 = monkeys.iter().map(|m| m.divisibility_test).product();
    let manage_worry = |worry| worry % modulus;

    let mut inspection_count = keep_away_activity_count(monkeys, manage_worry, 10_000);

    // Technically faster to just run a "max" with two buckets but my input only has 8 monkeys.
    inspection_count.sort_unstable();
    inspection_count.into_iter().rev().take(2).product()
}

fn keep_away_activity_count<F>(mut monkeys: Vec<Monkey>, manage_worry: F, rounds: usize) -> Vec<u64>
where
    F: Fn(i64) -> i64,
{
    let mut inspection_count = vec![0; monkeys.len()];
    for _ in 0..rounds {
        // Still waiting on `get_many_mut` ... might move to `nightly` or copy-paste it.
        for idx in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[idx].items);
            inspection_count[idx] += items.len() as u64;

            for item in items {
                let new_item = manage_worry(monkeys[idx].update_worry(item));
                let next_monkey = monkeys[idx].next_monkey(new_item);
                // Could use unchecked indexing here
                monkeys[next_monkey].items.push(new_item);
            }
        }
    }

    inspection_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1(EXAMPLE), 10605);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 151312);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part2(EXAMPLE), 2_713_310_158);
        }

        #[test]
        fn my_input() {
            assert_eq!(part2(INPUT), 51_382_025_916);
        }
    }
}
