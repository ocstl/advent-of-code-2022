use advent_of_code_2022::position::{Direction, Position};
use std::collections::HashSet;

const FILE: &str = "inputs/day9.txt";

type Instruction = (Direction, usize);

fn simulation<const N: usize>(instructions: &[Instruction]) -> usize {
    let mut knots = [Position::default(); N];
    let mut visited = HashSet::new();
    visited.insert(Position::default());

    for &(direction, steps) in instructions {
        for _ in 0..steps {
            knots[0] += direction;
            for k in 1..N {
                let d = knots[k - 1] - knots[k];
                // Advance towards the preceding knot if too far away.
                if d.maximum_norm() > 1 {
                    knots[k] += d.signum();
                    // Update the visited positions if the tail moves.
                    if k == N - 1 {
                        visited.insert(knots[k]);
                    }
                }
            }
        }
    }

    visited.len()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|instruction| {
            let (direction, steps) = instruction.split_once(' ').expect("Invalid instruction.");
            let direction =
                Direction::try_from(direction.chars().next().expect("Invalid instruction."))
                    .expect("Invalid instruction.");
            let steps: usize = steps.parse().expect("Invalid number of steps.");
            (direction, steps)
        })
        .collect();

    // Simulate your complete hypothetical series of motions. How many
    // positions does the tail of the rope visit at least once?
    let part1 = simulation::<2>(&instructions);
    println!("Part 1: {part1}");

    // Simulate your complete series of motions on a larger rope with ten knots.
    // How many positions does the tail of the rope visit at least once?
    let part2 = simulation::<10>(&instructions);
    println!("Part 2: {part2}");

    Ok(())
}
