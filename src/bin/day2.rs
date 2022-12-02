const FILE: &str = "inputs/day2.txt";

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            // This is for part1.
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => unreachable!("Wrong letter."),
        }
    }
}

impl Shape {
    fn choose_shape(outcome: Round, other: Shape) -> Self {
        match (outcome, other) {
            (Round::Loss, Shape::Rock) => Shape::Scissors,
            (Round::Loss, Shape::Paper) => Shape::Rock,
            (Round::Loss, Shape::Scissors) => Shape::Paper,
            (Round::Draw, Shape::Rock) => Shape::Rock,
            (Round::Draw, Shape::Paper) => Shape::Paper,
            (Round::Draw, Shape::Scissors) => Shape::Scissors,
            (Round::Win, Shape::Rock) => Shape::Paper,
            (Round::Win, Shape::Paper) => Shape::Scissors,
            (Round::Win, Shape::Scissors) => Shape::Rock,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Round {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<char> for Round {
    fn from(value: char) -> Self {
        match value {
            'X' => Round::Loss,
            'Y' => Round::Draw,
            'Z' => Round::Win,
            _ => unreachable!("Wrong letter."),
        }
    }
}

impl From<(Shape, Shape)> for Round {
    fn from(round: (Shape, Shape)) -> Self {
        match round {
            (Shape::Rock, Shape::Rock) => Round::Draw,
            (Shape::Rock, Shape::Paper) => Round::Loss,
            (Shape::Rock, Shape::Scissors) => Round::Win,
            (Shape::Paper, Shape::Rock) => Round::Win,
            (Shape::Paper, Shape::Paper) => Round::Draw,
            (Shape::Paper, Shape::Scissors) => Round::Loss,
            (Shape::Scissors, Shape::Rock) => Round::Loss,
            (Shape::Scissors, Shape::Paper) => Round::Win,
            (Shape::Scissors, Shape::Scissors) => Round::Draw,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rounds = std::fs::read_to_string(FILE)?;

    // What would your total score be if everything goes exactly according to
    // your strategy guide?
    let part1: u32 = rounds
        .lines()
        .map(|round| {
            let other = Shape::from(round.chars().next().expect("Missing first character."));
            let mine = Shape::from(round.chars().last().expect("Missing second character."));
            Round::from((mine, other)) as u32 + mine as u32
        })
        .sum();
    println!("Part1: {part1}");

    // Following the Elf's instructions for the second column, what would your
    // total score be if everything goes exactly according to your strategy
    // guide?
    let part2: u32 = rounds
        .lines()
        .map(|round| {
            let other = Shape::from(round.chars().next().expect("Missing first character."));
            let round = Round::from(round.chars().last().expect("Missing second character."));
            Shape::choose_shape(round, other) as u32 + round as u32
        })
        .sum();
    println!("Part2: {part2}");

    Ok(())
}
