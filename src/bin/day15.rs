use advent_of_code_2022::position::Position;
use advent_of_code_2022::range_extension::RangeExtension;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

const FILE: &str = "inputs/day15.txt";
type Beacon = Position;

#[derive(Debug, Clone, Copy)]
struct Sensor {
    position: Position,
    beacon: Beacon,
}

impl Sensor {
    pub fn row_coverage(self, row: isize) -> RangeInclusive<isize> {
        let coverage = self.position.manhattan_distance(self.beacon);
        let vertical_cost = (self.position.y() - row).abs();

        // Once we have accounted the number of 'steps' to reach the row, this
        // leaves us with that many steps to either side. Subtract one since
        // there are never two equidistant beacons.
        let horizontal_leeway = coverage - vertical_cost;
        RangeInclusive::new(
            self.position.x() - horizontal_leeway,
            self.position.x() + horizontal_leeway,
        )
    }
}

impl FromStr for Sensor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .split(&[' ', '=', ',', ':'])
            .filter_map(|v| v.parse::<isize>().ok());
        let position = Position::new(iter.next().unwrap(), iter.next().unwrap());
        let beacon = Position::new(iter.next().unwrap(), iter.next().unwrap());

        Ok(Self { position, beacon })
    }
}

#[derive(Debug, Default, Clone)]
struct Intervals {
    ranges: Vec<RangeInclusive<isize>>,
}

impl Intervals {
    pub fn new(start: isize, end: isize) -> Self {
        let ranges = std::iter::once(RangeInclusive::new(start, end))
            .filter(|r| !r.is_empty())
            .collect();
        Intervals { ranges }
    }
}

impl std::ops::SubAssign<RangeInclusive<isize>> for Intervals {
    fn sub_assign(&mut self, rhs: RangeInclusive<isize>) {
        if rhs.is_empty() {
            return;
        }

        let mut ranges = Vec::new();
        std::mem::swap(&mut ranges, &mut self.ranges);

        for range in ranges {
            if rhs.contains_range(&range) {
                continue;
            }

            if !rhs.overlaps(&range) {
                self.ranges.push(range);
                continue;
            }

            let (start, end) = range.into_inner();

            let first = RangeInclusive::new(start, end.min(rhs.start() - 1));
            if !first.is_empty() {
                self.ranges.push(first);
            }
            let second = RangeInclusive::new(start.max(rhs.end() + 1), end);
            if !second.is_empty() {
                self.ranges.push(second);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let sensors = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Sensor>, _>>()?;

    // Consult the report from the sensors you just deployed. In the row where
    // y=2000000, how many positions cannot contain a beacon?
    const ROW: isize = 2000000;
    let ranges: Vec<RangeInclusive<isize>> = sensors.iter().map(|s| s.row_coverage(ROW)).collect();
    let min_x = ranges
        .iter()
        .map(RangeInclusive::start)
        .min()
        .copied()
        .unwrap_or_default();
    let max_x = ranges
        .iter()
        .map(RangeInclusive::end)
        .max()
        .copied()
        .unwrap_or_default();
    // Remove spots already occupied by a beacon.
    let occupied: HashSet<isize> = sensors
        .iter()
        .filter_map(|s| {
            if s.beacon.y() == ROW {
                Some(s.beacon.x())
            } else {
                None
            }
        })
        .collect();
    let part1 = (min_x..=max_x)
        .filter(|x| !occupied.contains(x) && ranges.iter().any(|r| r.contains(x)))
        .count();

    println!("Part 1: {part1}");

    // Find the only possible position for the distress beacon. What is its
    // tuning frequency?
    const MAX: isize = 4_000_000;
    for y in 0..=MAX {
        let possibles = sensors.iter().fold(Intervals::new(0, MAX), |mut acc, s| {
            acc -= s.row_coverage(y);
            acc
        });

        if !possibles.ranges.is_empty() {
            let x = possibles.ranges[0].start();
            let part2 = x * MAX + y;
            println!("Part 2: {part2}");
            break;
        }
    }

    Ok(())
}
