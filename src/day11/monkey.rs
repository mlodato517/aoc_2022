#[derive(Debug)]
pub(crate) struct Monkey {
    pub(crate) items: Vec<i64>,
    operation: Operation,
    pub(crate) divisibility_test: i64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    pub fn update_worry(&self, worry: i64) -> i64 {
        match self.operation {
            Operation::Add(Value::Literal(n)) => worry + n,
            Operation::Multiply(Value::Literal(n)) => worry * n,
            Operation::Add(Value::Old) => worry + worry,
            Operation::Multiply(Value::Old) => worry * worry,
        }
    }
    pub fn next_monkey(&self, worry: i64) -> usize {
        if worry % self.divisibility_test == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
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

pub(crate) fn parse_block(block: &str) -> Monkey {
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
