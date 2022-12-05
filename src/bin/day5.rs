use std::str::FromStr;

const FILE: &str = "inputs/day5.txt";

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok());
        let count = iter.next().ok_or("Missing count.")?;
        // The stacks are 1-indexed.
        let from = iter.next().ok_or("Missing origin.")? - 1;
        let to = iter.next().ok_or("Missing destination.")? - 1;

        Ok(Instruction { count, from, to })
    }
}

#[derive(Debug, Clone, Copy)]
enum Crane {
    CrateMover9000,
    CrateMover9001,
}

fn move_crates(mut stacks: Stacks, instructions: &[Instruction], crane: Crane) -> Stacks {
    for &Instruction { count, from, to } in instructions {
        let idx = stacks[from].len() - count;
        let mut tail = stacks[from].split_off(idx);
        match crane {
            Crane::CrateMover9000 => {
                tail.reverse();
            }
            Crane::CrateMover9001 => (),
        }
        stacks[to].append(&mut tail);
    }

    stacks
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let (stacks, steps) = input.split_once("\n\n").expect("Bad format.");

    // Build the stacks. The last (first) line to allocate, the rest to fill.
    let mut iter = stacks
        .lines()
        .rev()
        .map(|line| line.chars().skip(1).step_by(4));

    let nbr_stacks = iter
        .by_ref()
        .next()
        .map(Iterator::count)
        .unwrap_or_default();
    let mut stacks = vec![Stack::new(); nbr_stacks];

    for line in iter {
        for (idx, c) in line.enumerate().filter(|(_, c)| c.is_alphabetic()) {
            stacks[idx].push(c);
        }
    }

    // Parse the instructions.
    let instructions = steps
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Instruction>, _>>()?;

    // After the rearrangement procedure completes, what crate ends up on top
    // of each stack?
    let part1: String = move_crates(stacks.clone(), &instructions, Crane::CrateMover9000)
        .iter()
        .filter_map(|stack| stack.last())
        .collect();
    println!("Part 1: {part1}");

    // After the rearrangement procedure completes, what crate ends up on top
    // of each stack?
    let part2: String = move_crates(stacks, &instructions, Crane::CrateMover9001)
        .iter()
        .filter_map(|stack| stack.last())
        .collect();
    println!("Part 2: {part2}");

    Ok(())
}
