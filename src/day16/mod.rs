use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Valve {
    idx: usize,
    rate: u64,
    tunnels: Vec<usize>,
}

pub fn part1(input: &str) -> u64 {
    let mut names_to_idx = HashMap::new();
    let mut valves = input
        .lines()
        .map(|line| {
            const VALVE_NAME_START: usize = 6;
            const VALVE_NAME_END: usize = 8;
            let name = &line[VALVE_NAME_START..VALVE_NAME_END];
            let len = names_to_idx.len();
            let idx = *names_to_idx.entry(name).or_insert(len);

            let mut split = line.split(';');

            const RATE_START: usize = 23;
            let rate = &split.next().expect("Should have rate")[RATE_START..];
            let rate = rate.parse().expect("Invalid rate value");

            let suffix = split.next().expect("Should have suffix");
            let mut suffix = suffix.split(' ');

            // <space> tunnel(s) lead to valve(s) XX, YY
            for _ in 0..5 {
                suffix.next();
            }

            let tunnels = suffix
                .map(|tunnel| {
                    let len = names_to_idx.len();
                    let idx = names_to_idx
                        .entry(tunnel.trim_end_matches(','))
                        .or_insert(len);
                    *idx
                })
                .collect();

            Valve { idx, rate, tunnels }
        })
        .collect::<Vec<_>>();
    valves.sort_unstable_by_key(|valve| valve.idx);

    // Make a new, complete graph from the previous one. The new edges will have weights
    // corresponding to how many edges were used to make that edge. Then we can get get from X to Z
    // and vice versa without having to remember that there's a Y in between.
    let mut adjacency_list = vec![Vec::new(); valves.len()];

    // BFS from each valve to find every other valve.
    for source in &valves {
        let edges = &mut adjacency_list[source.idx];

        let mut visited = vec![false; valves.len()];
        let mut frontier = vec![source];
        let mut new_frontier = Vec::new();
        let mut distance = 0;
        while !frontier.is_empty() {
            distance += 1;
            for node in frontier.drain(..) {
                for &dst in &node.tunnels {
                    if !visited[dst] {
                        visited[dst] = true;
                        let valve = &valves[dst];
                        new_frontier.push(valve);
                        edges.push((valve, distance));
                    }
                }
            }
            std::mem::swap(&mut frontier, &mut new_frontier);
        }
    }

    // Starting from "AA", basically brute force search every path of less than 30 (counting the
    // time it takes to open valves) to see which one releases the most pressure. Credit to Chris
    // Rice for this algorithm. Credit to me for however horribly it's implemented.
    let starting_idx = names_to_idx["AA"];
    let minutes_remaining = 30;
    let mut max_pressure = 0;
    for &(valve, distance) in &adjacency_list[starting_idx] {
        // Could use `.filter()` here but we can't easily use it in `max_pressure_from` and this is
        // nice and symmetric.
        if distance >= minutes_remaining || valve.rate == 0 {
            continue;
        }
        let mut open_valves = vec![false; valves.len()];
        open_valves[valve.idx] = true;
        let new_pressure = max_pressure_from(
            valve,
            minutes_remaining - distance,
            &adjacency_list,
            &mut open_valves,
        );
        max_pressure = max_pressure.max(new_pressure);
    }
    max_pressure
}

fn max_pressure_from(
    valve: &Valve,
    mut minutes_remaining: u64,
    adjacency_list: &Vec<Vec<(&Valve, u64)>>,
    open_valves: &mut Vec<bool>,
) -> u64 {
    minutes_remaining -= 1;
    let pressure_relieved = valve.rate * minutes_remaining;

    let mut max_pressure = pressure_relieved;
    for &(valve, distance) in &adjacency_list[valve.idx] {
        if distance >= minutes_remaining || valve.rate == 0 || open_valves[valve.idx] {
            continue;
        }
        open_valves[valve.idx] = true;
        let new_pressure = max_pressure_from(
            valve,
            minutes_remaining - distance,
            adjacency_list,
            open_valves,
        );
        open_valves[valve.idx] = false;
        max_pressure = max_pressure.max(pressure_relieved + new_pressure);
    }
    max_pressure
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    const INPUT: &str = include_str!("./input.txt");

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            assert_eq!(part1(EXAMPLE), 1651);
        }

        #[test]
        fn my_input() {
            assert_eq!(part1(INPUT), 1850);
        }
    }
}
//
