use advent_of_code_2022::grid::{Direction, Position};
use std::collections::HashMap;
use std::str::FromStr;

const FILE: &str = "inputs/day14.txt";
const SAND_SOURCE: Position = Position::new(500, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Sand,
}

fn draw_line(a: Position, b: Position) -> impl Iterator<Item = Position> {
    let ax = a.x();
    let ay = a.y();
    let bx = b.x();
    let by = b.y();

    // In this case, we're assuming that we are dealing with horizontal or
    // vertical lines. We can survive straight diagonals though.
    (ax.min(bx)..=(ax.max(bx)))
        .flat_map(move |x| (ay.min(by)..=ay.max(by)).map(move |y| Position::new(x, y)))
}

#[derive(Debug, Clone)]
struct Cave {
    cave: HashMap<Position, Tile>,
}

impl Cave {
    pub fn fill(&mut self, sand_source: Position) -> usize {
        let deepest = self.cave.keys().map(|p| p.y()).max().unwrap_or_default();

        let mut sand = sand_source;

        while sand.y() <= deepest {
            let down = (sand + Direction::Down).expect("Can't fail.");
            if !self.cave.contains_key(&down) {
                sand = down;
                continue;
            }

            let dl = (sand + Direction::DownLeft).expect("Overflow to the left.");
            if !self.cave.contains_key(&dl) {
                sand = dl;
                continue;
            }

            let dr = (sand + Direction::DownRight).expect("Can't fail.");
            if !self.cave.contains_key(&dr) {
                sand = dr;
                continue;
            }

            self.cave.insert(sand, Tile::Sand);
            sand = sand_source;
        }

        self.cave.values().filter(|&&t| t == Tile::Sand).count()
    }

    pub fn fill_part2(&mut self, sand_source: Position) -> usize {
        let last_empty_row = self.cave.keys().map(|p| p.y()).max().unwrap_or_default() + 1;
        let mut sand = sand_source;

        while self.cave.get(&sand_source).is_none() {
            if sand.y() == last_empty_row {
                self.cave.insert(sand, Tile::Sand);
                sand = sand_source;
                continue;
            }

            let down = (sand + Direction::Down).expect("Can't fail.");
            if !self.cave.contains_key(&down) {
                sand = down;
                continue;
            }

            let dl = (sand + Direction::DownLeft).expect("Overflow to the left.");
            if !self.cave.contains_key(&dl) {
                sand = dl;
                continue;
            }

            let dr = (sand + Direction::DownRight).expect("Can't fail.");
            if !self.cave.contains_key(&dr) {
                sand = dr;
                continue;
            }

            self.cave.insert(sand, Tile::Sand);
            sand = sand_source;
        }

        self.cave.values().filter(|&&t| t == Tile::Sand).count()
    }
}

impl FromStr for Cave {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave = HashMap::new();

        for line in s.lines() {
            let points: Vec<Position> = line
                .split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').expect("Invalid input.");
                    Position::new(
                        x.parse().expect("Invalid x position."),
                        y.parse().expect("Invalid y position."),
                    )
                })
                .collect();

            for pair in points.windows(2) {
                for p in draw_line(pair[0], pair[1]) {
                    cave.insert(p, Tile::Rock);
                }
            }
        }

        Ok(Cave { cave })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut cave: Cave = input.parse()?;

    // Using your scan, simulate the falling sand. How many units of sand come
    // to rest before sand starts flowing into the abyss below?
    let part1 = cave.fill(SAND_SOURCE);
    println!("Part 1: {part1}");

    // Using your scan, simulate the falling sand until the source of the sand
    // becomes blocked. How many units of sand come to rest?
    // We can continue using the same cave, since we've only started to
    // overflow towards the bottom.
    let part2 = cave.fill_part2(SAND_SOURCE);
    println!("Part 2: {part2}");

    Ok(())
}
