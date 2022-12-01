use std::cmp::Reverse;

const FILE: &str = "inputs/day1.txt";

type Calories = u64;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut calories = std::fs::read_to_string(FILE)?
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<Calories>()).sum())
        .collect::<Result<Vec<Calories>, _>>()?;

    // Find the Elf carrying the most Calories. How many total Calories is
    // that Elf carrying?
    calories.sort_unstable_by_key(|calories| Reverse(*calories));
    let part1 = calories.first().copied().unwrap_or_default();
    println!("Part 1: {part1}");

    // Find the top three Elves carrying the most Calories. How many Calories
    // are those Elves carrying in total?
    let part2 = calories.iter().take(3).sum::<Calories>();
    println!("Part 2: {part2}");

    Ok(())
}
