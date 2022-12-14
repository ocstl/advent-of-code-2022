use std::cmp::Ordering;
use std::str::FromStr;

const FILE: &str = "inputs/day13.txt";

type Value = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(Value),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Integer(left), Packet::Integer(right)) => left.partial_cmp(right),
            (Packet::List(left), Packet::List(right)) => left.partial_cmp(right),
            (Packet::Integer(left), Packet::List(right)) => {
                vec![Packet::Integer(*left)].partial_cmp(right)
            }
            (Packet::List(left), Packet::Integer(right)) => {
                left.partial_cmp(&vec![Packet::Integer(*right)])
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_list<T: Iterator<Item = u8>>(iter: &mut T) -> Packet {
    let mut values = Vec::new();
    let mut buffer = None;
    while let Some(c) = iter.next() {
        match c {
            b'[' => values.push(parse_list(iter.by_ref())),
            b']' => {
                if let Some(v) = buffer.take() {
                    values.push(Packet::Integer(v));
                }
                break;
            }
            b',' => {
                if let Some(v) = buffer.take() {
                    values.push(Packet::Integer(v));
                }
            }
            b'0'..=b'9' => {
                let v = buffer.unwrap_or_default();
                buffer = Some(v * 10 + c - b'0');
            }
            _ => unreachable!("{}", char::from(c)),
        }
    }

    Packet::List(values)
}

impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.bytes();
        Ok(parse_list(&mut iter))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let pairs: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|pair| {
            let mut iter = pair.lines();
            let left = iter.next().unwrap_or_default().parse().expect("Bad input.");
            let right = iter.next().unwrap_or_default().parse().expect("Bad input.");
            (left, right)
        })
        .collect();

    // Determine which pairs of packets are already in the right order.
    // What is the sum of the indices of those pairs?
    let part1: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, (left, right))| if left <= right { Some(idx + 1) } else { None })
        .sum();
    println!("Part 1: {part1}");

    // Organize all of the packets into the correct order.
    // What is the decoder key for the distress signal?
    let first_divider: Packet = "[[2]]".parse().expect("Infallible.");
    let second_divider: Packet = "[[6]]".parse().expect("Infallible.");

    let mut packets: Vec<Packet> = pairs
        .into_iter()
        .flat_map(|pair| [pair.0, pair.1].into_iter())
        .collect();
    packets.push(first_divider.clone());
    packets.push(second_divider.clone());
    packets.sort_unstable();

    let part2 = (packets
        .iter()
        .position(|p| p == &first_divider)
        .unwrap_or_default()
        + 1)
        * (packets
            .iter()
            .position(|p| p == &second_divider)
            .unwrap_or_default()
            + 1);
    println!("Part 2: {part2}");

    Ok(())
}
