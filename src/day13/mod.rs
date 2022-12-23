use std::cmp::Ordering;

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, pair)| {
            let mut pair = pair.split('\n');
            let first = pair.next().expect("Each pair should have two items");
            let second = pair.next().expect("Each pair should have two items");

            // The top level packets are always lists
            let first = &first[1..first.len() - 1];
            let second = &second[1..second.len() - 1];

            let first = parse(first);
            let second = parse(second);

            let in_order = first < second;

            in_order.then_some(i + 1)
        })
        .sum()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PacketData {
    List(Vec<PacketData>),
    Int(u8),
}

// TODO we should be able to yield items and validate order as we go
fn parse(s: &str) -> PacketData {
    let mut items = vec![];
    let mut item_start = 0;

    while item_start < s.len() {
        let suffix = &s[item_start..];

        if suffix.starts_with('[') {
            // Find closing brace
            let bytes = suffix.as_bytes();
            let mut bracket_count = 1;
            let mut idx = 1;
            while idx < bytes.len() {
                if bytes[idx] == b'[' {
                    bracket_count += 1;
                } else if bytes[idx] == b']' {
                    bracket_count -= 1;
                    if bracket_count == 0 {
                        break;
                    }
                }
                idx += 1;
            }
            // Parse the item without the square brackets
            let inner_list = &suffix[1..idx];
            let item = parse(inner_list);
            items.push(item);

            item_start += inner_list.len() + 3;
        } else {
            let int = suffix.split([',', ']']).next().expect("Int never ends!");
            items.push(PacketData::Int(int.parse().expect("Invalid int!")));

            item_start += int.len() + 1;
        }
    }
    PacketData::List(items)
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketData::List(first), PacketData::List(second)) => {
                for (first, second) in first.iter().zip(second) {
                    match first.cmp(second) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {}
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                first.len().cmp(&second.len())
            }

            // TODO wasteful allocations! Could use `SmallVec` but likely there's a better way like
            // just comparing the number to the first item in the other list or something.
            (f @ PacketData::List(_), PacketData::Int(s)) => {
                f.cmp(&PacketData::List(vec![PacketData::Int(*s)]))
            }
            (PacketData::Int(f), s @ PacketData::List(_)) => {
                PacketData::List(vec![PacketData::Int(*f)]).cmp(s)
            }

            (PacketData::Int(first), PacketData::Int(second)) => first.cmp(second),
        }
    }
}

pub fn part2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // The top level packets are always lists
            parse(&line[1..line.len() - 1])
        })
        .collect();

    let divider_packet_1 = PacketData::List(vec![PacketData::Int(2)]);
    let divider_packet_2 = PacketData::List(vec![PacketData::Int(6)]);
    packets.push(divider_packet_1.clone());
    packets.push(divider_packet_2.clone());

    packets.sort_unstable();

    let first_packet_idx = packets
        .iter()
        .position(|packet| packet == &divider_packet_1)
        .expect("Lost divider packet 1!");
    let second_packet_idx = packets
        .iter()
        .position(|packet| packet == &divider_packet_2)
        .expect("Lost divider packet 2!");

    (first_packet_idx + 1) * (second_packet_idx + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1(EXAMPLE), 13);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 4809);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part2(EXAMPLE), 140);
        }

        #[test]
        fn my_input() {
            assert_eq!(part2(INPUT), 22600);
        }
    }
}
