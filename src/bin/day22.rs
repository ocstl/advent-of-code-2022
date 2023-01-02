use advent_of_code_2022::grid::{Direction, Position};
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

const FILE: &str = "inputs/day22.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Wall),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone)]
struct MonkeyMap {
    height: usize,
    width: usize,
    map: HashMap<Position, Tile>,
}

impl MonkeyMap {
    pub fn iter(&self) -> Iter<'_, Position, Tile> {
        self.map.iter()
    }

    pub fn get(&self, k: &Position) -> Option<&Tile> {
        self.map.get(k)
    }
}

impl FromStr for MonkeyMap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: HashMap<Position, Tile> = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.char_indices().filter_map(move |(col, tile)| {
                    tile.try_into()
                        .ok()
                        .map(|tile| (Position::new(col + 1, row + 1), tile))
                })
            })
            .collect();
        let height = map
            .keys()
            .map(|position| position.y())
            .max()
            .unwrap_or_default();
        let width = map
            .keys()
            .map(|position| position.x())
            .max()
            .unwrap_or_default();

        Ok(MonkeyMap { height, width, map })
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Steps(u8),
    TurnLeft,
    TurnRight,
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut buffer = None;
    for c in s.trim().bytes() {
        match c {
            b'L' => {
                if let Some(v) = buffer.take() {
                    instructions.push(Instruction::Steps(v));
                }
                instructions.push(Instruction::TurnLeft);
            }
            b'R' => {
                if let Some(v) = buffer.take() {
                    instructions.push(Instruction::Steps(v));
                }
                instructions.push(Instruction::TurnRight);
            }
            b'0'..=b'9' => {
                let b = buffer.get_or_insert(0);
                *b = *b * 10 + c - b'0';
            }
            _ => unreachable!("{}", char::from(c)),
        }
    }

    // Make sure the buffer is empty before returning the instructions!!!
    if let Some(v) = buffer.take() {
        instructions.push(Instruction::Steps(v));
    }

    instructions
}

fn password(position: Position, direction: Direction) -> usize {
    1000 * position.y()
        + 4 * position.x()
        + match direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
            _ => unreachable!(),
        }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let (map, instructions) = input.split_once("\n\n").unwrap_or_default();
    let map: MonkeyMap = map.parse()?;
    let instructions = parse_instructions(instructions);

    // Follow the path given in the monkeys' notes. What is the final password?
    let mut direction = Direction::Right;
    let mut position = map
        .iter()
        .filter_map(|(position, tile)| {
            if *tile == Tile::Open && position.y() == 1 {
                Some(position)
            } else {
                None
            }
        })
        .min_by_key(|position| position.x())
        .copied()
        .unwrap_or_default();

    for instruction in instructions {
        match instruction {
            Instruction::Steps(steps) => {
                'outer: for _ in 0..steps {
                    let mut next = (position + direction).expect("Can't fail.");
                    loop {
                        match map.get(&next) {
                            Some(Tile::Open) => {
                                position = next;
                                break;
                            }
                            Some(Tile::Wall) => {
                                break 'outer;
                            }
                            None => match (direction, (next.x(), next.y())) {
                                (Direction::Up, (x, 0)) => {
                                    next = Position::new(x, map.height + 1);
                                }
                                (Direction::Down, (x, y)) if y > map.height => {
                                    next = Position::new(x, 0);
                                }
                                (Direction::Left, (0, y)) => {
                                    next = Position::new(map.width + 1, y);
                                }
                                (Direction::Right, (x, y)) if x > map.width => {
                                    next = Position::new(0, y);
                                }
                                _ => (),
                            },
                        }
                        next = (next + direction).expect("Can't fail.");
                    }
                }
            }
            Instruction::TurnLeft => direction = direction.rotate_left(),
            Instruction::TurnRight => direction = direction.rotate_right(),
        }
    }

    let part1 = password(position, direction);
    println!("Part 1: {part1}");

    Ok(())
}
