use advent_of_code_2022::position::{
    Direction, Position, DIRECTIONS, DOWN, DOWN_LEFT, DOWN_RIGHT, LEFT, RIGHT, UP, UP_LEFT,
    UP_RIGHT,
};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const FILE: &str = "inputs/day23.txt";
const MOVES: [[Direction; 3]; 4] = [
    ([DOWN_LEFT, DOWN, DOWN_RIGHT]),
    ([UP_LEFT, UP, UP_RIGHT]),
    ([UP_LEFT, LEFT, DOWN_LEFT]),
    ([UP_RIGHT, RIGHT, DOWN_RIGHT]),
];
const ELF: char = '#';

#[derive(Debug, Default, Clone)]
struct Grove {
    round: usize,
    elves: HashSet<Position>,
}

impl Iterator for Grove {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // The elves will consider NORTH (UP) first, then SOUTH (DOWN) first,
        // etc.
        let moves = MOVES.into_iter().cycle().skip(self.round).take(4);

        let mut counts: HashMap<Position, u32> = HashMap::new();
        let mut no_neighbours: usize = 0;
        let propositions: HashMap<Position, Position> = self
            .elves
            .iter()
            .filter_map(|&elf| {
                // If we have no neighbors, don't try to move. Otherwise, find
                // the first proposition available.
                if !DIRECTIONS
                    .into_iter()
                    .any(|d| self.elves.contains(&(elf + d)))
                {
                    no_neighbours += 1;
                    None
                } else {
                    moves.clone().find_map(|checks| {
                        if !checks.into_iter().any(|d| self.elves.contains(&(elf + d))) {
                            let new_position = elf + checks[1];
                            *counts.entry(new_position).or_default() += 1;
                            Some((elf, new_position))
                        } else {
                            None
                        }
                    })
                }
            })
            .collect();

        // We're done once the elves no longer try to move.
        if no_neighbours == self.elves.len() {
            None
        } else {
            let mut new_elves = self
                .elves
                .iter()
                .map(|elf| {
                    propositions
                        .get(elf)
                        .filter(|proposed| counts.get(proposed).copied().unwrap_or_default() == 1)
                        .copied()
                        .unwrap_or(*elf)
                })
                .collect();
            std::mem::swap(&mut new_elves, &mut self.elves);
            self.round += 1;
            Some(self.round)
        }
    }
}

impl FromStr for Grove {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.char_indices().filter_map(move |(col, tile)| {
                    if tile == ELF {
                        Some(Position::new(col as isize, row as isize))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Ok(Grove { round: 0, elves })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut elves: Grove = input.parse()?;

    // Simulate the Elves' process and find the smallest rectangle that
    // contains the Elves after 10 rounds. How many empty ground tiles does
    // that rectangle contain?
    elves.nth(9);
    let min_x = elves.elves.iter().map(|p| p.x()).min().unwrap_or_default();
    let max_x = elves.elves.iter().map(|p| p.x()).max().unwrap_or_default();
    let min_y = elves.elves.iter().map(|p| p.y()).min().unwrap_or_default();
    let max_y = elves.elves.iter().map(|p| p.y()).max().unwrap_or_default();
    let part1 = ((1 + max_x - min_x) * (1 + max_y - min_y)) - elves.elves.len() as isize;
    println!("Part 1: {part1}");

    // Figure out where the Elves need to go. What is the number of the first
    // round where no Elf moves?
    let part2 = elves.last().unwrap_or_default() + 1;
    println!("Part 2: {part2}");

    Ok(())
}
