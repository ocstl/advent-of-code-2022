use advent_of_code_2022::position::{Direction, Position, DOWN, LEFT, RIGHT};
use std::collections::{BTreeSet, HashMap, HashSet};

const FILE: &str = "inputs/day17.txt";
const HORIZONTAL: [Direction; 4] = [
    Direction::new(0, 0),
    Direction::new(1, 0),
    Direction::new(2, 0),
    Direction::new(3, 0),
];
const CROSS: [Direction; 5] = [
    Direction::new(1, 0),
    Direction::new(0, 1),
    Direction::new(1, 1),
    Direction::new(2, 1),
    Direction::new(1, 2),
];
const CORNER: [Direction; 5] = [
    Direction::new(0, 0),
    Direction::new(1, 0),
    Direction::new(2, 0),
    Direction::new(2, 1),
    Direction::new(2, 2),
];
const VERTICAL: [Direction; 4] = [
    Direction::new(0, 0),
    Direction::new(0, 1),
    Direction::new(0, 2),
    Direction::new(0, 3),
];
const SQUARE: [Direction; 4] = [
    Direction::new(0, 0),
    Direction::new(1, 0),
    Direction::new(0, 1),
    Direction::new(1, 1),
];
const SHAPES: [&[Direction]; 5] = [&HORIZONTAL, &CROSS, &CORNER, &VERTICAL, &SQUARE];

#[derive(Debug, Default, Clone, Copy)]
struct Rock {
    position: Position,
    shape: &'static [Direction],
}

impl Rock {
    pub fn new(position: Position, shape: &'static [Direction]) -> Self {
        Self { position, shape }
    }

    pub fn positions(self) -> impl Iterator<Item = Position> {
        self.shape.iter().map(move |&d| self.position + d)
    }
}

impl std::ops::Add<Direction> for Rock {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Rock::new(self.position + rhs, self.shape)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let jets: Vec<Direction> = input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => LEFT,
            '>' => RIGHT,
            _ => unreachable!("Bad character."),
        })
        .collect();

    // How many units tall will the tower of rocks be after 2022 rocks have
    // stopped falling?
    let mut iter_jets = jets.iter().copied().cycle();
    let mut iter_shapes = SHAPES.iter().cycle();
    let mut cave = HashSet::new();
    let mut highest = 0;

    for _ in 0..2022 {
        let mut current = Rock::new(Position::new(2, highest + 4), iter_shapes.next().unwrap());

        loop {
            // Push it, then check that it is not blocked.
            let lateral = current + iter_jets.next().unwrap();
            if lateral
                .positions()
                .all(|p| p.x() >= 0 && p.x() <= 6 && !cave.contains(&p))
            {
                current = lateral;
            }

            // Let gravity do the work. If it is blocked, add the positions of the
            // rocks to the cave.
            let downward = current + DOWN;
            if downward
                .positions()
                .any(|p| p.y() == 0 || cave.contains(&p))
            {
                for p in current.positions() {
                    cave.insert(p);
                    highest = highest.max(p.y());
                }
                break;
            } else {
                current = downward;
            }
        }
    }
    println!("Part 1: {highest}");

    // How tall will the tower be after 1000000000000 rocks have stopped?
    // Ah well, that is a different beast. Let's try and find a repetition.
    const TOTAL_ROCKS: usize = 1_000_000_000_000;
    let mut iter_jets = jets.iter().copied().cycle();
    let nbr_jets = jets.len();
    let nbr_rocks = SHAPES.len();
    let mut jet_idx = 0;
    let mut rocks_idx = 0;
    let mut floor: BTreeSet<Position> = (0..7).map(|x| Position::new(x, 0)).collect();
    let mut cumulative = 0;
    let mut floor_patterns = HashMap::new();

    let mut previous_pattern = None;
    while previous_pattern.is_none() {
        for shape in SHAPES {
            let mut current = Rock::new(
                Position::new(2, floor.iter().map(|p| p.y()).max().unwrap_or_default() + 4),
                shape,
            );

            loop {
                // Push it, then check that it is not blocked.
                jet_idx = (jet_idx + 1) % nbr_jets;
                let lateral = current + iter_jets.next().unwrap();
                if lateral
                    .positions()
                    .all(|p| (0..7).contains(&p.x()) && !floor.contains(&p))
                {
                    current = lateral;
                }

                // Let gravity do the work. If it is blocked, add the positions of the
                // rocks to the cave.
                let downward = current + DOWN;
                if downward.positions().any(|p| floor.contains(&p)) {
                    floor.extend(current.positions());
                    break;
                } else {
                    current = downward;
                }
            }
        }

        let raise_floor = (0..7)
            .map(|x| {
                floor
                    .iter()
                    .filter(|p| p.x() == x)
                    .map(|p| p.y())
                    .max()
                    .unwrap_or_default()
            })
            .min()
            .unwrap_or_default();
        cumulative += raise_floor;
        let d = Direction::new(0, -raise_floor);
        floor = floor
            .into_iter()
            .map(|e| e + d)
            .filter(|p| p.y() >= 0)
            .collect();
        rocks_idx += nbr_rocks;
        previous_pattern = floor_patterns.insert((jet_idx, floor.clone()), (rocks_idx, cumulative));
    }

    // Now that we have a pattern, we can use the number of rocks between both
    // to skip ahead (quite) a bit.
    let (previous_idx, previous_cumulative) = previous_pattern.unwrap_or_default();
    let height_change = cumulative - previous_cumulative;
    let rocks_change = rocks_idx - previous_idx;
    let nbr_repeats = (TOTAL_ROCKS - rocks_idx) / rocks_change;

    rocks_idx += nbr_repeats * rocks_change;
    cumulative += (nbr_repeats as isize) * height_change;
    let rocks_target = TOTAL_ROCKS + previous_idx - rocks_idx;

    // Find the remaining height adjustment.
    let ((_, final_floor), (_, final_cum)) = floor_patterns
        .iter()
        .find(|(_, (rocks, _))| *rocks == rocks_target)
        .unwrap();
    let part2 = cumulative
        + (final_cum - previous_cumulative)
        + final_floor.iter().map(|p| p.y()).max().unwrap_or_default();
    println!("Part 2: {part2}");

    Ok(())
}
