use std::fmt::Display;
use std::str::FromStr;

const FILE: &str = "inputs/day25.txt";

#[derive(Debug, Default, Clone, Copy)]
struct Snafu(i64);

impl Snafu {
    const BASE: i64 = 5;
}

impl std::ops::Add for Snafu {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Snafu(self.0 + rhs.0)
    }
}

impl FromStr for Snafu {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut base = 1;
        let mut result = 0;

        for c in s.chars().rev() {
            match c {
                '=' => result += -2 * base,
                '-' => result += -base,
                '0' => (),
                '1' => result += base,
                '2' => result += 2 * base,
                _ => Err("Invalid character")?,
            }
            base *= Self::BASE;
        }

        Ok(Snafu(result))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut s = Vec::new();
        let mut n = self.0;
        let mut carry = 0;
        while n > 0 {
            let current = n % Self::BASE + carry;
            carry = 0;
            match current {
                -2 => s.push('='),
                -1 => s.push('-'),
                0 => s.push('0'),
                1 => s.push('1'),
                2 => s.push('2'),
                3 => {
                    s.push('=');
                    carry = 1;
                }
                4 => {
                    s.push('-');
                    carry = 1;
                }
                5 => {
                    s.push('0');
                    carry = 1;
                }
                _ => unreachable!("{}", current),
            }
            n /= Self::BASE;
        }

        write!(f, "{}", s.into_iter().rev().collect::<String>())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let snafus = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Snafu>, _>>()?;

    // The Elves are starting to get cold. What SNAFU number do you supply to
    // Bob's console?
    let part1: Snafu = snafus
        .into_iter()
        .fold(Snafu(0), |acc, current| Snafu(acc.0 + current.0));
    println!("Part 1: {part1}");

    Ok(())
}
