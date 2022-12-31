const FILE: &str = "inputs/day20.txt";
const DECRYPTION_KEY: i64 = 811589153;
const OFFSETS: [i64; 3] = [1000, 2000, 3000];

fn decrypt(numbers: &[i64], rounds: u32) -> i64 {
    let length = numbers.len() as i64;
    let mut positions: Vec<i64> = (0..length).collect();

    // Mixing.
    for _ in 0..rounds {
        for (idx, number) in numbers.iter().enumerate() {
            let start = positions[idx];
            let end = (start + number).rem_euclid(length - 1);
            positions.iter_mut().for_each(|p| {
                if *p > start {
                    *p -= 1
                };
                if *p >= end {
                    *p += 1
                };
            });
            positions[idx] = end;
        }
    }

    // Decryption.
    let idx_zero = positions[numbers.iter().position(|&n| n == 0).unwrap_or_default()];
    OFFSETS
        .into_iter()
        .map(|offset| {
            let target = (idx_zero + offset) % numbers.len() as i64;
            let idx = positions
                .iter()
                .position(|&p| p == target)
                .unwrap_or_default();
            numbers[idx]
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let numbers = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()?;

    // Mix your encrypted file exactly once. What is the sum of the three
    // numbers that form the grove coordinates?
    let part1 = decrypt(&numbers, 1);
    println!("Part 1: {part1}");

    // Apply the decryption key and mix your encrypted file ten times.
    // What is the sum of the three numbers that form the grove coordinates?
    let part2 = decrypt(
        &numbers
            .iter()
            .map(|n| n * DECRYPTION_KEY)
            .collect::<Vec<i64>>(),
        10,
    );
    println!("Part 2: {part2}");

    Ok(())
}
