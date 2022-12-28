use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

const FILE: &str = "inputs/day18.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn neighbors(self) -> [Self; 6] {
        [
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x + 1, self.y, self.z),
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            Cube::new(self.x, self.y, self.z - 1),
            Cube::new(self.x, self.y, self.z + 1),
        ]
    }
}

trait Droplet {
    fn surface_area(&self) -> usize;
    fn exterior_surface_area(&self) -> usize;
}

impl Droplet for HashSet<Cube> {
    fn surface_area(&self) -> usize {
        self.iter()
            .flat_map(|cube| cube.neighbors())
            .filter(|cube| !self.contains(cube))
            .count()
    }

    fn exterior_surface_area(&self) -> usize {
        // Make sure our exploration zone has at least one free cube on all sides, but no more.
        let min_x = self.iter().map(|cube| cube.x).min().unwrap_or_default();
        let max_x = self.iter().map(|cube| cube.x).max().unwrap_or_default();
        let min_y = self.iter().map(|cube| cube.y).min().unwrap_or_default();
        let max_y = self.iter().map(|cube| cube.y).max().unwrap_or_default();
        let min_z = self.iter().map(|cube| cube.z).min().unwrap_or_default();
        let max_z = self.iter().map(|cube| cube.z).max().unwrap_or_default();
        let exploration_zone = |cube: &Cube| {
            (min_x - 1..=max_x + 1).contains(&cube.x)
                && (min_y - 1..=max_y + 1).contains(&cube.y)
                && (min_z - 1..=max_z + 1).contains(&cube.z)
        };

        // Starting at the very edge, keep spreading the water/steam unless we
        // hit an edge (or lava).
        let mut to_visit = vec![Cube::new(min_x - 1, min_y - 1, min_z - 1)];
        let mut exterior = HashSet::new();
        exterior.insert(Cube::new(min_x - 1, min_y - 1, min_z - 1));
        let mut count = 0;

        while let Some(cube) = to_visit.pop() {
            // If the 'neighbor' is lava, we have a surface area. Otherwise,
            // add it if we haven't visited (or planned to visit) it before.
            for neighbor in cube.neighbors().into_iter().filter(exploration_zone) {
                if self.contains(&neighbor) {
                    count += 1;
                } else if exterior.insert(neighbor) {
                    to_visit.push(neighbor);
                }
            }
        }

        count
    }
}

impl FromStr for Cube {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter.next().unwrap_or_default().parse()?;
        let y = iter.next().unwrap_or_default().parse()?;
        let z = iter.next().unwrap_or_default().parse()?;

        Ok(Cube { x, y, z })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let cubes = input
        .lines()
        .map(str::parse)
        .collect::<Result<HashSet<Cube>, _>>()?;

    // What is the surface area of your scanned lava droplet?
    let part1 = cubes.surface_area();
    println!("Part 1: {part1}");

    // What is the exterior surface area of your scanned lava droplet?
    let part2 = cubes.exterior_surface_area();
    println!("Part 2: {part2}");

    Ok(())
}
