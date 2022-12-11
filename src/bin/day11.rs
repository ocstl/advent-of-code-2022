use std::str::FromStr;

const FILE: &str = "inputs/day11.txt";

type WorryLevel = u64;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(WorryLevel),
    Mul(WorryLevel),
    AddSelf,
    MulSelf,
}

impl Op {
    fn apply(self, worry_level: WorryLevel) -> WorryLevel {
        match self {
            Op::Add(w) => worry_level + w,
            Op::Mul(w) => worry_level * w,
            Op::AddSelf => worry_level * 2,
            Op::MulSelf => worry_level * worry_level,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    value: WorryLevel,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn apply(self, worry_level: WorryLevel) -> usize {
        if worry_level % self.value == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey<const RELIEF: WorryLevel> {
    items: Vec<WorryLevel>,
    op: Op,
    test: Test,
    items_inspected: usize,
}

impl<const RELIEF: WorryLevel> Monkey<RELIEF> {
    fn throws(&mut self) -> Vec<(usize, WorryLevel)> {
        self.items_inspected += self.items.len();

        let result = self
            .items
            .iter()
            .map(|item| {
                let w = self.op.apply(*item) / RELIEF;
                let d = self.test.apply(w);
                (d, w)
            })
            .collect();
        self.items.clear();

        result
    }

    fn catch(&mut self, item: WorryLevel) {
        self.items.push(item);
    }
}

impl<const RELIEF: WorryLevel> FromStr for Monkey<RELIEF> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Skip the first line (monkey id).
        let mut iter = s.lines().skip(1);

        // Items.
        let items = iter
            .next()
            .map(|items| {
                items
                    .split_whitespace()
                    .filter_map(|word| word.trim_matches(',').parse::<WorryLevel>().ok())
                    .collect()
            })
            .unwrap_or_default();

        // Worry level operation.
        let op = iter
            .next()
            .map(|op| {
                let mut terms = op.split_whitespace().rev();

                match terms.next() {
                    Some("old") => match terms.next() {
                        Some("+") => Op::AddSelf,
                        Some("*") => Op::MulSelf,
                        _ => unreachable!(),
                    },
                    Some(v) => {
                        let operand = v.parse::<WorryLevel>().expect("Invalid operand.");
                        match terms.next() {
                            Some("+") => Op::Add(operand),
                            Some("*") => Op::Mul(operand),
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            })
            .expect("Missing operation.");

        // Test.
        let test_value: WorryLevel = iter
            .next()
            .and_then(|line| line.split_whitespace().last())
            .and_then(|v| v.parse().ok())
            .expect("Missing test value.");
        let if_true = iter
            .next()
            .and_then(|line| line.split_whitespace().last())
            .and_then(|v| v.parse().ok())
            .expect("Missing destination.");
        let if_false = iter
            .next()
            .and_then(|line| line.split_whitespace().last())
            .and_then(|v| v.parse().ok())
            .expect("Missing destination.");

        let test = Test {
            value: test_value,
            if_true,
            if_false,
        };

        Ok(Monkey {
            items,
            op,
            test,
            items_inspected: 0,
        })
    }
}

#[derive(Debug, Clone)]
struct KeepAwayGame<const RELIEF: WorryLevel> {
    monkeys: Vec<Monkey<RELIEF>>,
    modulus: WorryLevel,
}

impl<const RELIEF: WorryLevel> KeepAwayGame<RELIEF> {
    fn round(&mut self) -> &mut Self {
        for idx in 0..self.monkeys.len() {
            let throws = self.monkeys[idx].throws();
            for (idx, item) in throws {
                self.monkeys[idx].catch(item % self.modulus);
            }
        }

        self
    }

    fn monkey_business_level(&self) -> usize {
        let mut inspections: Vec<usize> = self.monkeys.iter().map(|m| m.items_inspected).collect();
        inspections.sort_unstable_by_key(|i| std::cmp::Reverse(*i));

        inspections.into_iter().take(2).product()
    }
}

impl<const RELIEF: WorryLevel> FromStr for KeepAwayGame<RELIEF> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .split("\n\n")
            .map(str::parse)
            .collect::<Result<Vec<Monkey<RELIEF>>, _>>()?;

        // We will quickly overflow when we can't get no relief. Looking at the
        // tests, we are dealing with prime numbers only, so we can use their
        // product to keep worry levels manageable.
        let modulus = monkeys.iter().map(|monkey| monkey.test.value).product();
        Ok(KeepAwayGame { monkeys, modulus })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    // Figure out which monkeys to chase by counting how many items they
    // inspect over 20 rounds. What is the level of monkey business after 20
    // rounds of stuff-slinging simian shenanigans?
    let mut game: KeepAwayGame<3> = input.parse()?;
    for _ in 0..20 {
        game.round();
    }
    let part1 = game.monkey_business_level();
    println!("Part 1: {part1}");

    // Worry levels are no longer divided by three after each item is inspected;
    // you'll need to find another way to keep your worry levels manageable.
    // Starting again from the initial state in your puzzle input, what is the
    // level of monkey business after 10000 rounds?
    let mut game: KeepAwayGame<1> = input.parse()?;
    for _ in 1..=10000 {
        game.round();
    }
    let part2 = game.monkey_business_level();
    println!("Part 2: {part2}");

    Ok(())
}
