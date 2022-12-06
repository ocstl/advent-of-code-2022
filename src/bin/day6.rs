const FILE: &str = "inputs/day6.txt";

#[derive(Debug, Clone, Copy)]
enum Marker {
    StartOfPacket = 4,
    StartOfMessage = 14,
}

fn marker_position(message: &str, marker: Marker) -> Result<usize, &'static str> {
    message
        .as_bytes()
        .windows(marker as usize)
        .position(|window| (0..window.len()).all(|idx| !window[idx + 1..].contains(&window[idx])))
        // Add marker size, since we start at that character.
        .map(|idx| idx + marker as usize)
        .ok_or("No marker.")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    // How many characters need to be processed before the first
    // start-of-packet marker is detected?
    let part1 = marker_position(&input, Marker::StartOfPacket)?;
    println!("Part 1: {part1}");

    // How many characters need to be processed before the first
    // start-of-message marker is detected?
    let part2 = marker_position(&input, Marker::StartOfMessage)?;
    println!("Part 2: {part2}");

    Ok(())
}
