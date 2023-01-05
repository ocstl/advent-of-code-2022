use advent_of_code_2022::grid::{Direction, Position};
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

const FILE: &str = "inputs/day24.txt";
const DIRECTIONS: [Direction; 4] = [
    Direction::Down,
    Direction::Right,
    Direction::Up,
    Direction::Left,
];

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    position: Position,
    direction: Direction,
}

impl Blizzard {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn next(self) -> Self {
        Self::new(
            (self.position + self.direction).expect("Underflow."),
            self.direction,
        )
    }
}

#[derive(Debug, Clone)]
struct Valley {
    height: usize,
    width: usize,
    state: Vec<Blizzard>,
    states: Vec<HashSet<Position>>,
}

impl Valley {
    pub fn get_state(&mut self, time: usize) -> &HashSet<Position> {
        // Iterate the state of the valley until we have the state we need.
        while self.states.len() <= time {
            self.next();
        }

        &self.states[time]
    }

    pub fn next(&mut self) {
        for blizzard in self.state.iter_mut() {
            *blizzard = blizzard.next();
            // If we hit a wall, start again at the end.
            match (
                blizzard.direction,
                blizzard.position.x(),
                blizzard.position.y(),
            ) {
                (Direction::Up, x, 0) => {
                    *blizzard = Blizzard::new(Position::new(x, self.height - 2), Direction::Up)
                }
                (Direction::Down, x, y) if y == self.height - 1 => {
                    *blizzard = Blizzard::new(Position::new(x, 1), Direction::Down)
                }
                (Direction::Left, 0, y) => {
                    *blizzard = Blizzard::new(Position::new(self.width - 2, y), Direction::Left)
                }
                (Direction::Right, x, y) if x == self.width - 1 => {
                    *blizzard = Blizzard::new(Position::new(1, y), Direction::Right)
                }
                _ => (),
            }
        }

        // Save the states, so we do not have to recalculate them each time.
        self.states.push(
            self.state
                .iter()
                .map(|blizzard| blizzard.position)
                .collect(),
        );
    }
}

impl FromStr for Valley {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().len();
        let state: Vec<Blizzard> = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.char_indices()
                    .filter_map(move |(col, tile)| match tile {
                        '^' => Some(Blizzard::new(Position::new(col, row), Direction::Up)),
                        'v' => Some(Blizzard::new(Position::new(col, row), Direction::Down)),
                        '<' => Some(Blizzard::new(Position::new(col, row), Direction::Left)),
                        '>' => Some(Blizzard::new(Position::new(col, row), Direction::Right)),
                        _ => None,
                    })
            })
            .collect();
        let states = vec![state.iter().map(|blizzard| blizzard.position).collect()];

        Ok(Self {
            height,
            width,
            state,
            states,
        })
    }
}

fn fastest_path(start: Position, end: Position, time: usize, valley: &mut Valley) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((time, start));

    // Since the path is irrelevant, we can filter out possibilities we've
    // already seen based on the position and the number of steps to reach it.
    let mut visited = HashSet::new();
    visited.insert((time, start));

    while let Some((steps, position)) = to_visit.pop_front() {
        // We've reached the end.
        if position == end {
            return steps;
        }

        // We've been caught in a blizzard. Bad.
        if valley.get_state(steps).contains(&position) {
            continue;
        }

        // We're in a wall. Bad as well.
        if !(position == start || position == end)
            && ((position.x() == 0)
                || (position.y() == 0)
                || (position.x() >= valley.width - 1)
                || (position.y() >= valley.height - 1))
        {
            continue;
        }

        // Stick around, then move around.
        to_visit.push_back((steps + 1, position));
        to_visit.extend(
            DIRECTIONS
                .into_iter()
                .filter_map(|d| position + d)
                .filter_map(|p| {
                    let s = steps + 1;
                    if visited.insert((s, p)) {
                        Some((s, p))
                    } else {
                        None
                    }
                }),
        );
    }

    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut valley: Valley = input.parse()?;

    // What is the fewest number of minutes required to avoid the blizzards and
    // reach the goal?
    let start = Position::new(1, 0);
    let end = Position::new(valley.width - 2, valley.height - 1);
    let part1 = fastest_path(start, end, 0, &mut valley);
    println!("Part 1: {part1}");

    // What is the fewest number of minutes required to reach the goal, go back
    // to the start, then reach the goal again?
    // Since we can stick around and not move, we can find the shortest numbers
    // of steps from the start to the end, then use that number of steps as the
    // starting time at the end for the second leg, then the shortest to walk
    // again to the end.
    let part2 = fastest_path(end, start, part1, &mut valley);
    let part2 = fastest_path(start, end, part2, &mut valley);
    println!("Part 2: {part2}");

    Ok(())
}
