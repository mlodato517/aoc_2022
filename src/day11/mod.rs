pub fn part1(input: &str) -> u64 {
    let mut monkeys = parse_monkeys(input);
    let mut inspection_count = vec![0; monkeys.len()];
    for _ in 0..20 {
        // Still waiting on `get_many_mut` ... might move to `nightly` or copy-paste it.
        for idx in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[idx].items);
            inspection_count[idx] += items.len() as u64;

            for item in items {
                let new_item = monkeys[idx].update_worry(item) / 3;
                let next_monkey = if monkeys[idx].test_divisibility(new_item) {
                    monkeys[idx].true_monkey
                } else {
                    monkeys[idx].false_monkey
                };
                // Could use unchecked indexing here
                monkeys[next_monkey].items.push(new_item);
            }
        }
    }

    // Technically faster to just run a "max" with two buckets but my input only has 8 monkeys.
    inspection_count.sort_unstable();
    inspection_count.into_iter().rev().take(2).product()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisibility_test: i64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn update_worry(&self, worry: i64) -> i64 {
        match self.operation {
            Operation::Add(Value::Literal(n)) => worry + n,
            Operation::Multiply(Value::Literal(n)) => worry * n,
            Operation::Add(Value::Old) => worry + worry,
            Operation::Multiply(Value::Old) => worry * worry,
        }
    }
    fn test_divisibility(&self, worry: i64) -> bool {
        worry % self.divisibility_test == 0
    }
}

#[derive(Debug)]
enum Operation {
    Add(Value),
    Multiply(Value),
}

#[derive(Debug)]
enum Value {
    Literal(i64),
    Old,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(parse_block).collect()
}

fn parse_block(block: &str) -> Monkey {
    let mut lines = block.lines().skip(1);

    let items_line = lines.next().expect("Should have starting items");
    let items = parse_items(items_line);

    let op_line = lines.next().expect("Should have operation");
    let operation = parse_operation(op_line);

    let div_test_line = lines.next().expect("Should have test");
    let divisibility_test = parse_divisibility(div_test_line);

    let true_line = lines.next().expect("Should have 'if true' line");
    let true_monkey = parse_next_monkey(true_line);

    let false_line = lines.next().expect("Should have 'if false' line");
    let false_monkey = parse_next_monkey(false_line);

    Monkey {
        items,
        operation,
        true_monkey,
        false_monkey,
        divisibility_test,
    }
}

// These are all of the form 'Starting items: N1, N2, N3'
fn parse_items(line: &str) -> Vec<i64> {
    let nums = line.split(": ").nth(1).expect("Should have list");
    nums.split(", ")
        .map(|n| n.parse().expect("Invalid item number"))
        .collect()
}

// These are all of the form 'Operation: new = old X Y'
// where X is `+` or `*` and `Y` is a number or 'old'.
fn parse_operation(line: &str) -> Operation {
    let mut suffix = line.split(' ').rev();
    let value = suffix.next().expect("Should have value");
    let operation = suffix.next().expect("Should have operation");
    let value = match value {
        "old" => Value::Old,
        value => Value::Literal(value.parse().expect("Invalid literal")),
    };
    match operation {
        "*" => Operation::Multiply(value),
        "+" => Operation::Add(value),
        _ => panic!("Invalid operation of {operation}!"),
    }
}

// These are all of the form 'Test: divisible by N'
fn parse_divisibility(line: &str) -> i64 {
    let n = line.split(' ').rev().next().expect("Should have literal");
    n.parse().expect("Invalid number for divisibility test")
}

// These are all of the form 'If true/false: throw to monkey N'
fn parse_next_monkey(line: &str) -> usize {
    let monkey_num = line
        .split(' ')
        .rev()
        .next()
        .expect("Should have next monkey number");
    monkey_num.parse().expect("Invalid number for monkey")
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
}
