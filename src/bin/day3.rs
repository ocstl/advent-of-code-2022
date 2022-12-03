use std::collections::HashSet;

const FILE: &str = "inputs/day3.txt";

type Item = u8;

trait Priority {
    fn priority(self) -> u32;
}

impl Priority for Item {
    fn priority(self) -> u32 {
        u32::from(if self.is_ascii_lowercase() {
            self - b'a' + 1
        } else {
            self - b'A' + 27
        })
    }
}

pub trait RuckSack {
    fn common_items(&self) -> HashSet<Item>;
}

impl RuckSack for str {
    fn common_items(&self) -> HashSet<Item> {
        let items = self.as_bytes();
        let (first, second) = items.split_at(items.len() / 2);

        &first.iter().copied().collect::<HashSet<Item>>()
            & &second.iter().copied().collect::<HashSet<Item>>()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rucksacks = std::fs::read_to_string(FILE)?;

    // Find the item type that appears in both compartments of each rucksack.
    // What is the sum of the priorities of those item types?
    let part1: u32 = rucksacks
        .lines()
        .flat_map(RuckSack::common_items)
        .map(Item::priority)
        .sum();
    println!("Part1: {part1}");

    // Find the item type that corresponds to the badges of each three-Elf
    // group. What is the sum of the priorities of those item types?
    let part2: u32 = rucksacks
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .flat_map(|group| {
            group
                .iter()
                .map(|rucksack| rucksack.bytes().collect::<HashSet<Item>>())
                .reduce(|accum, item| &accum & &item)
                .unwrap_or_default()
        })
        .map(Item::priority)
        .sum();
    println!("Part2: {part2}");

    Ok(())
}
