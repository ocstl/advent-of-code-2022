use std::ops::RangeInclusive;

const FILE: &str = "inputs/day4.txt";

type SectionId = u32;

trait RangeExtension {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl<U: Sized + PartialOrd> RangeExtension for RangeInclusive<U> {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || other.contains(self.start())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let assignments: Vec<(RangeInclusive<SectionId>, RangeInclusive<SectionId>)> =
        std::fs::read_to_string(FILE)?
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(',').expect("Bad input.");
                let left = left.split_once('-').expect("Bad input.");
                let left = RangeInclusive::new(
                    left.0.parse::<SectionId>().expect("Bad input."),
                    left.1.parse::<SectionId>().expect("Bad input."),
                );
                let right = right.split_once('-').expect("Bad input.");
                let right = RangeInclusive::new(
                    right.0.parse::<SectionId>().expect("Bad input."),
                    right.1.parse::<SectionId>().expect("Bad input."),
                );
                (left, right)
            })
            .collect();

    // In how many assignment pairs does one range fully contain the other?
    let part1 = assignments
        .iter()
        .filter(|(r0, r1)| r0.contains_range(r1) || r1.contains_range(r0))
        .count();
    println!("Part 1: {part1}");

    // In how many assignment pairs do the ranges overlap?
    let part2 = assignments
        .iter()
        .filter(|(r0, r1)| r0.overlaps(r1))
        .count();
    println!("Part 2: {part2}");

    Ok(())
}
