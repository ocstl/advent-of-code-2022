use advent_of_code_2022::grid::{Grid, Position};
use std::collections::{HashSet, VecDeque};

const FILE: &str = "inputs/day12.txt";

type Map = Grid<u8>;

fn parse_input(input: &str) -> (Position, Position, Map) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap_or_default().len();
    let mut start = Position::default();
    let mut end = Position::default();
    let grid = input
        .lines()
        .flat_map(str::bytes)
        .enumerate()
        .map(|(idx, c)| match c {
            b'S' => {
                start = Position::new(idx % width, idx / width);
                0
            }
            b'E' => {
                end = Position::new(idx % width, idx / width);
                25
            }
            c => c - b'a',
        })
        .collect();

    (
        start,
        end,
        Map::new(height, width, grid).expect("Shouldn't fail."),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let (start, end, grid) = parse_input(&input);

    // What is the fewest steps required to move from your current position to
    // the location that should get the best signal?
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::from([(0, start, grid.get(start).expect("Can't fail."))]);
    visited.insert(start);

    while let Some((steps, position, &height)) = to_visit.pop_front() {
        if position == end {
            println!("Part 1: {steps}");
            break;
        }

        for new_position in position.neighbours() {
            // Check that we are in the grid.
            if let Some(new_height) = grid.get(new_position) {
                // That the step is small enough.
                if height + 1 >= *new_height {
                    // And filter out those we have reached before.
                    if visited.insert(new_position) {
                        to_visit.push_back((steps + 1, new_position, new_height));
                    }
                }
            }
        }
    }

    // What is the fewest steps required to move starting from any square with
    // elevation a to the location that should get the best signal?
    // We'll just start the end, reversing the condition on the upwards steps
    // as well as the final condition (height 0 == 'a').
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::from([(0, end, grid.get(end).expect("Can't fail."))]);
    visited.insert(end);

    while let Some((steps, position, &height)) = to_visit.pop_front() {
        if height == 0 {
            println!("Part 2: {steps}");
            break;
        }

        for new_position in position.neighbours() {
            // Check that we are in the grid.
            if let Some(new_height) = grid.get(new_position) {
                // That the step is small enough.
                if new_height + 1 >= height {
                    // And filter out those we have reached before.
                    if visited.insert(new_position) {
                        to_visit.push_back((steps + 1, new_position, new_height));
                    }
                }
            }
        }
    }

    Ok(())
}
