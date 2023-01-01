use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

const FILE: &str = "inputs/day21.txt";

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => Err("Invalid operation.")?,
        })
    }
}

#[derive(Debug, Clone)]
enum Monkey {
    Number(i64),
    Operation(String, String, Operation),
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = s.parse() {
            Ok(Monkey::Number(number))
        } else {
            let mut iter = s.split_whitespace();
            let a = iter.next().unwrap_or_default().to_string();
            let op = iter.next().unwrap_or_default().parse()?;
            let b = iter.next().unwrap_or_default().to_string();
            Ok(Monkey::Operation(a, b, op))
        }
    }
}

#[derive(Debug, Clone)]
struct Monkeys(HashMap<String, Monkey>);

impl Monkeys {
    pub fn entry(&mut self, name: String) -> Entry<'_, String, Monkey> {
        self.0.entry(name)
    }

    pub fn insert(&mut self, name: String, monkey: Monkey) {
        self.0.insert(name, monkey);
    }

    pub fn get(&self, name: &str) -> i64 {
        match self.0.get(name).expect("No monkey by that name.") {
            Monkey::Number(n) => *n,
            Monkey::Operation(a, b, op) => {
                let a = self.get(a);
                let b = self.get(b);
                match op {
                    Operation::Add => a + b,
                    Operation::Subtract => a - b,
                    Operation::Multiply => a * b,
                    Operation::Divide => a / b,
                }
            }
        }
    }
}

impl FromStr for Monkeys {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut h = HashMap::new();

        for line in s.lines() {
            let (name, rest) = line.split_once(": ").unwrap_or_default();
            h.insert(name.to_string(), rest.parse()?);
        }

        Ok(Monkeys(h))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut monkeys: Monkeys = input.parse()?;

    // However, your actual situation involves considerably more monkeys.
    // What number will the monkey named root yell?
    let part1 = monkeys.get("root");
    println!("Part 1: {part1}");

    // What number do you yell to pass root's equality test?
    // Equals just means no *difference*, so we'll just replace the operation
    // for root with a difference, and try to find 0.
    const ROOT: &str = "root";
    const HUMN: &str = "humn";
    monkeys.entry(ROOT.to_string()).and_modify(|root| {
        if let Monkey::Operation(_, _, op) = root {
            *op = Operation::Subtract;
        }
    });

    // Try a binary search. Eyeballing it shows that, in this case, ROOT goes
    // down as HUMN increases. Alternatively, we could reverse the order of
    // operands of ROOT instead.
    // Note that we get plateaus when using integers, and that, while not
    // explicitly mentioned in the problem, we need to return the smallest
    // valid number (confirmed when using floats).
    let mut left = 0;
    let mut right = i64::MAX >> 6;
    while left != right {
        let mid = (left + right) / 2;
        monkeys.insert(HUMN.to_string(), Monkey::Number(mid));
        let root = monkeys.get(ROOT);
        if root > 0 {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    println!("Part 2: {left}");

    Ok(())
}
