use std::collections::HashSet;
use std::str::FromStr;

const FILE: &str = "inputs/day8.txt";

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct Grid {
    height: usize,
    width: usize,
    grid: Vec<u32>,
}

impl Grid {
    pub fn row(&self, idy: usize) -> impl Iterator<Item = &u32> {
        self.grid.iter().skip(idy * self.width).take(self.width)
    }

    pub fn column(&self, idx: usize) -> impl Iterator<Item = &u32> {
        self.grid
            .iter()
            .skip(idx)
            .step_by(self.height)
            .take(self.height)
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &u32>> {
        (0..self.height).map(|idy| self.row(idy))
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &u32>> {
        (0..self.width).map(|idx| self.column(idx))
    }
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().map_or(0, |line| line.chars().count());

        let grid: Vec<u32> = s
            .lines()
            .flat_map(|line| line.chars().filter_map(|c| c.to_digit(10)))
            .collect();

        // Check that we have the correct dimensions.
        if grid.len() != height * width {
            Err("Invalid input.")
        } else {
            Ok(Grid {
                height,
                width,
                grid,
            })
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let grid: Grid = input.parse()?;

    // Consider your map; how many trees are visible from outside the grid?
    let row_positions: HashSet<Position> = grid
        .rows()
        .enumerate()
        .flat_map(|(idy, row)| {
            let row: Vec<u32> = row.copied().collect();
            // First forward.
            let mut trees = row.iter().enumerate();
            let (idx, first) = trees.next().unwrap_or((0, &0));

            let forward = std::iter::once((idy, idx)).chain(trees.scan(
                (idx, *first),
                move |(idx, top), (current, tree)| {
                    if tree > top {
                        *idx = current;
                        *top = *tree;
                    }
                    Some((idy, *idx))
                },
            ));

            // Now backward.
            let mut trees = row.iter().enumerate().rev();
            let (idx, first) = trees.next().unwrap_or((0, &0));

            let backward = std::iter::once((idy, idx)).chain(trees.scan(
                (idx, *first),
                move |(idx, top), (current, tree)| {
                    if tree > top {
                        *idx = current;
                        *top = *tree;
                    }
                    Some((idy, *idx))
                },
            ));
            forward.chain(backward).collect::<HashSet<Position>>()
        })
        .collect();

    let col_positions: HashSet<Position> = grid
        .columns()
        .enumerate()
        .flat_map(|(idx, col)| {
            let col: Vec<u32> = col.copied().collect();
            // First forward.
            let mut trees = col.iter().enumerate();
            let (idy, first) = trees.next().unwrap_or((0, &0));

            let forward = std::iter::once((idy, idx)).chain(trees.scan(
                (idy, *first),
                move |(idy, top), (current, tree)| {
                    if tree > top {
                        *idy = current;
                        *top = *tree;
                    }
                    Some((*idy, idx))
                },
            ));

            // Now backward.
            let mut trees = col.iter().enumerate().rev();
            let (idy, first) = trees.next().unwrap_or((0, &0));

            let backward = std::iter::once((idy, idx)).chain(trees.scan(
                (idy, *first),
                move |(idy, top), (current, tree)| {
                    if tree > top {
                        *idy = current;
                        *top = *tree;
                    }
                    Some((*idy, idx))
                },
            ));
            forward.chain(backward).collect::<HashSet<Position>>()
        })
        .collect();
    let visible_trees: HashSet<Position> = &row_positions | &col_positions;
    let part1 = visible_trees.len();
    println!("Part 1: {part1}");

    // Consider each tree on your map. What is the highest scenic score
    // possible for any tree?
    let part2 = grid
        .grid
        .iter()
        .enumerate()
        .map(|(p, &tree)| {
            let idy = p / grid.width;
            let idx = p % grid.width;

            let row: Vec<u32> = grid.row(idy).copied().collect();
            let col: Vec<u32> = grid.column(idx).copied().collect();

            // Scenic score.
            (row.iter()
                .take(idx)
                .rev()
                .position(|&h| h >= tree)
                .map_or(idx, |c| c + 1))
                * (row
                    .iter()
                    .skip(idx + 1)
                    .position(|&h| h >= tree)
                    .map_or(grid.width - idx - 1, |c| c + 1))
                * (col
                    .iter()
                    .take(idy)
                    .rev()
                    .position(|&h| h >= tree)
                    .map_or(idy, |c| c + 1))
                * (col
                    .iter()
                    .skip(idy + 1)
                    .position(|&h| h >= tree)
                    .map_or(grid.height - idy - 1, |c| c + 1))
        })
        .max()
        .unwrap_or_default();
    println!("Part 2: {part2}");

    Ok(())
}
