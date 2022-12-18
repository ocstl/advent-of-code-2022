use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::str::FromStr;

const FILE: &str = "inputs/day16.txt";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Valve {
    name: String,
    flow_rate: u64,
    tunnel: Vec<String>,
}

impl FromStr for Valve {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let name = iter.nth(1).unwrap_or_default().to_string();
        let flow_rate = u64::from(
            iter.nth(2)
                .map(|s| {
                    s.bytes()
                        .filter(|b| b.is_ascii_digit())
                        .fold(0, |acc, b| acc * 10 + b - b'0')
                })
                .unwrap_or_default(),
        );
        let tunnel = iter
            .rev()
            .take_while(|&s| !s.starts_with("valve"))
            .map(|s| s.replace(',', ""))
            .collect();

        Ok(Valve {
            name,
            flow_rate,
            tunnel,
        })
    }
}

fn part1(
    time_remaining: u64,
    current: &Valve,
    open_valves: HashSet<&Valve>,
    valves: &HashMap<&Valve, Vec<(&Valve, u64)>>,
) -> u64 {
    valves
        .get(current)
        .unwrap()
        .iter()
        .map(|(valve, steps)| {
            if *steps >= time_remaining || open_valves.contains(valve) {
                0
            } else {
                let mut new_open = open_valves.clone();
                new_open.insert(valve);
                let time_remaining = time_remaining - steps;
                time_remaining * valve.flow_rate + part1(time_remaining, valve, new_open, valves)
            }
        })
        .max()
        .unwrap_or_default()
}

fn part2<'valves>(
    initial_time: u64,
    current: &'valves Valve,
    valves: &'valves HashMap<&Valve, Vec<(&Valve, u64)>>,
) -> u64 {
    // We'll generate all the possible paths, with the best pressure outcome.
    let mut optimal_paths = HashMap::new();
    let mut to_visit = vec![(initial_time, 0, current, BTreeSet::new())];

    while let Some((time_remaining, pressure, current, open_valves)) = to_visit.pop() {
        let e = optimal_paths.entry(open_valves.clone()).or_default();
        *e = pressure.max(*e);

        for (valve, steps) in valves.get(current).unwrap() {
            if *steps >= time_remaining || open_valves.contains(valve) {
                continue;
            }

            let time_remaining = time_remaining - steps;
            let mut open_valves = open_valves.clone();
            open_valves.insert(valve);
            to_visit.push((
                time_remaining,
                pressure + time_remaining * valve.flow_rate,
                valve,
                open_valves,
            ));
        }
    }

    // We can create pairs of paths, eliminating those where two paths share
    // an open valve (which would double count some valves). Then, we only need
    // to find the best pair.
    optimal_paths
        .iter()
        .flat_map(|(path, pressure)| {
            optimal_paths
                .iter()
                .map(move |(elephant, p2)| (path, elephant, pressure + p2))
        })
        .filter_map(|(path, elephant, pressure)| {
            if path.is_disjoint(elephant) {
                Some(pressure)
            } else {
                None
            }
        })
        .max()
        .unwrap_or_default()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|line| {
            let valve: Valve = line.parse().expect("Bad input.");
            (valve.name.to_string(), valve)
        })
        .collect();

    // We can cut down on the size of the problem by skipping broken valves and
    // adjusting the travel time. This will also eliminate some backwards steps.
    let mut connected_valves = HashMap::new();
    for (name, valve) in &valves {
        let mut visited = HashSet::new();
        visited.insert(name);
        visited.extend(valve.tunnel.iter());
        let mut to_visit: VecDeque<(&Valve, u64)> = valve
            .tunnel
            .iter()
            .map(|s| (valves.get(s).unwrap(), 1))
            .collect();

        let mut connections = Vec::new();
        while let Some((valve, steps)) = to_visit.pop_front() {
            if valve.flow_rate > 0 {
                // Add 1 to account for the time it takes to open the valve.
                connections.push((valve, steps + 1));
            }

            let next_valves = valve
                .tunnel
                .iter()
                .filter(|v| visited.insert(v))
                .map(|v| (valves.get(v).unwrap(), steps + 1));
            to_visit.extend(next_valves);
        }

        connected_valves.insert(valve, connections);
    }

    // Work out the steps to release the most pressure in 30 minutes.
    // What is the most pressure you can release?
    const START: &str = "AA";
    let part1 = part1(
        30,
        valves.get(START).unwrap(),
        HashSet::new(),
        &connected_valves,
    );
    println!("Part 1: {part1}");

    // With you and an elephant working together for 26 minutes, what is the
    // most pressure you could release?
    let part2 = part2(26, valves.get(START).unwrap(), &connected_valves);
    println!("Part 2: {part2}");

    Ok(())
}
