use std::str::FromStr;

const FILE: &str = "inputs/day10.txt";

type Value = i64;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(Value),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        match iter.next() {
            Some("noop") => Ok(Instruction::Noop),
            Some("addx") => iter
                .next()
                .and_then(|value| value.parse().ok())
                .map(Instruction::Addx)
                .ok_or("Invalid instruction."),
            _ => Err("Invalid instruction."),
        }
    }
}

fn register(instructions: &[Instruction]) -> impl Iterator<Item = i64> + '_ {
    std::iter::once(0)
        .chain(instructions.iter().flat_map(|instruction| {
            std::iter::once(0_i64).chain(
                match instruction {
                    Instruction::Noop => None,
                    Instruction::Addx(value) => Some(*value),
                }
                .into_iter(),
            )
        }))
        .scan(1_i64, |current, value| {
            *current += value;
            Some(*current)
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let instructions = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()?;

    // Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and
    // 220th cycles. What is the sum of these six signal strengths?
    const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let part1: i64 = register(&instructions)
        .enumerate()
        .filter_map(|(cycle, value)| {
            // Offset by 1, since we want the values at the end of the cycles.
            let cycle = cycle + 1;
            if CYCLES.contains(&cycle) {
                Some(value * cycle as i64)
            } else {
                None
            }
        })
        .sum();
    println!("Part 1: {part1}");

    // Render the image given by your program. What eight capital letters
    // appear on your CRT?
    const HEIGHT: i64 = 6;
    const WIDTH: i64 = 40;
    let mut iter = register(&instructions);

    println!("Part 2:");
    for _ in 0..HEIGHT {
        let line: String = (0..WIDTH)
            .zip(iter.by_ref())
            .map(|(h, v)| if (h - v).abs() <= 1 { '#' } else { '.' })
            .collect();
        println!("{line}");
    }

    Ok(())
}
