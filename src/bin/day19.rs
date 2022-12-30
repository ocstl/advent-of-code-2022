use std::str::FromStr;

const FILE: &str = "inputs/day19.txt";

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Ore {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

type Ores = [usize; 4];
type Cost = [usize; 3];

trait Resources {
    fn consume(self, cost: Cost) -> Option<Self>
    where
        Self: Sized;
    fn produce(&mut self, other: Self);
}

impl Resources for Ores {
    fn consume(mut self, cost: Cost) -> Option<Self> {
        for (ore, cost) in self.iter_mut().zip(cost) {
            *ore = ore.checked_sub(cost)?;
        }

        Some(self)
    }

    fn produce(&mut self, other: Self) {
        for (ore, robot) in self.iter_mut().zip(other) {
            *ore += robot;
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    id: usize,
    costs: [Cost; 4],
}

impl FromStr for Blueprint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (blueprint, robots) = s.split_once(": ").unwrap_or_default();
        let id = blueprint
            .split_whitespace()
            .flat_map(str::parse)
            .next()
            .unwrap_or_default();

        // Robots always have the same types of costs: ores only for ore and
        // clay robots, ore and clay for obsidian robots, ore and obsidian for
        // geode robots.
        let mut costs = robots.split_whitespace().flat_map(str::parse);
        let costs = [
            [costs.next().unwrap_or_default(), 0, 0],
            [costs.next().unwrap_or_default(), 0, 0],
            [
                costs.next().unwrap_or_default(),
                costs.next().unwrap_or_default(),
                0,
            ],
            [
                costs.next().unwrap_or_default(),
                0,
                costs.next().unwrap_or_default(),
            ],
        ];

        Ok(Blueprint { id, costs })
    }
}

#[derive(Debug, Copy, Clone)]
struct Factory<'blueprint> {
    time: usize,
    blueprint: &'blueprint Blueprint,
    ores: Ores,
    robots: Ores,
}

impl<'blueprint> Factory<'blueprint> {
    pub fn new(blueprint: &'blueprint Blueprint) -> Self {
        Self {
            time: 0,
            blueprint,
            ores: Ores::default(),
            robots: [1, 0, 0, 0],
        }
    }

    pub fn pass(mut self) -> Self {
        self.time -= 1;
        self.ores.produce(self.robots);
        self
    }

    pub fn max_geodes(mut self, minutes: usize) -> usize {
        self.time = minutes;

        // Assuming we can build 1 geode robot per minute for the remaining
        // time, we would produce ((n - 1) * n) / 2 geodes.
        let max_mineable_geodes: Vec<usize> = (0..=minutes)
            .map(|m| (m.saturating_sub(1) * m) / 2)
            .collect();

        // We can prune some branches by not constructing more robots than can
        // be used. Since we can only produce one robot per minute, there is no
        // point in having more ore-collecting robots than the highest cost.
        let max_ores = [
            self.blueprint
                .costs
                .iter()
                .map(|c| c[Ore::Ore as usize])
                .max()
                .unwrap_or_default(),
            self.blueprint
                .costs
                .iter()
                .map(|c| c[Ore::Clay as usize])
                .max()
                .unwrap_or_default(),
            self.blueprint
                .costs
                .iter()
                .map(|c| c[Ore::Obsidian as usize])
                .max()
                .unwrap_or_default(),
            0,
        ];

        let mut to_visit = vec![self];
        let mut geodes = 0;

        while let Some(mut factory) = to_visit.pop() {
            // There is no point in waiting another minute, as building a robot
            // will change nothing.
            if factory.time == 1 {
                geodes = geodes
                    .max(factory.ores[Ore::Geode as usize] + factory.robots[Ore::Geode as usize]);
                continue;
            }

            // If we can't beat the current maximum in the remaining time, quit.
            if factory.ores[Ore::Geode as usize]
                + (factory.time * factory.robots[Ore::Geode as usize])
                + max_mineable_geodes[factory.time]
                <= geodes
            {
                continue;
            }

            // Prioritize building geode-cracking robots.
            if let Some(ores) = factory
                .ores
                .consume(factory.blueprint.costs[Ore::Geode as usize])
            {
                factory.ores = ores;
                factory = factory.pass();
                factory.robots[Ore::Geode as usize] += 1;
                to_visit.push(factory);
                continue;
            }

            // Otherwise, try doing nothing and building all useful robots
            // (other than geode-cracking robots).
            to_visit.push(factory.pass());
            for (robot, &cost) in factory.blueprint.costs.iter().take(3).enumerate() {
                if max_ores[robot] > factory.robots[robot] {
                    if let Some(mut ores) = factory.ores.consume(cost) {
                        ores.produce(factory.robots);
                        let mut robots = factory.robots;
                        robots[robot] += 1;

                        to_visit.push(Factory {
                            time: factory.time - 1,
                            blueprint: factory.blueprint,
                            robots,
                            ores,
                        })
                    }
                }
            }
        }

        geodes
    }

    pub fn quality_level(self, minutes: usize) -> usize {
        self.blueprint.id * self.max_geodes(minutes)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let blueprints = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Blueprint>, _>>()?;

    // Determine the quality level of each blueprint using the largest number
    // of geodes it could produce in 24 minutes. What do you get if you add up
    // the quality level of all of the blueprints in your list?
    let part1: usize = blueprints
        .iter()
        .map(|blueprint| Factory::new(blueprint).quality_level(24))
        .sum();
    println!("Part 1: {part1}");

    // Don't worry about quality levels; instead, just determine the largest
    // number of geodes you could open using each of the first three blueprints.
    // What do you get if you multiply these numbers together?
    let part2: usize = blueprints
        .iter()
        .take(3)
        .map(|blueprint| Factory::new(blueprint).max_geodes(32))
        .product();
    println!("Part 2: {part2}");

    Ok(())
}
