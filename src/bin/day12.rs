use std::collections::{HashSet, VecDeque};

const FILE: &str = "inputs/day12.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn neighbours(self) -> impl Iterator<Item = Position> {
        self.x
            .checked_sub(1)
            .map(|x| Position::new(x, self.y))
            .into_iter()
            .chain(Some(Position::new(self.x + 1, self.y)).into_iter())
            .chain(
                self.y
                    .checked_sub(1)
                    .map(|y| Position::new(self.x, y))
                    .into_iter(),
            )
            .chain(Some(Position::new(self.x, self.y + 1)))
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Grid {
    height: usize,
    width: usize,
    grid: Vec<u8>,
}

impl Grid {
    pub fn get(&self, position: Position) -> Option<&u8> {
        if position.x < self.width {
            self.grid.get(position.x + position.y * self.width)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> (Position, Position, Grid) {
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
        Grid {
            height,
            width,
            grid,
        },
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
