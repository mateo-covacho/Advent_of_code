use std::fs;

// Opponent
// A for Rock
// B for Paper,
// C for Scissors

// ME
// X Lose
// Y Draw,
// Z Win

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut final_score: i64 = 0;

    enum Choices {
        Rock,
        Paper,
        Scissors,
        None,
    }

    for line in contents.lines() {
        let mut round_points: i64 = 0;

        // What is opponents choice
        let opponent_input;
        match line.chars().nth(0).unwrap() {
            'A' => opponent_input = Choices::Rock,
            'B' => opponent_input = Choices::Paper,
            'C' => opponent_input = Choices::Scissors,
            _ => opponent_input = Choices::None,
        }

        let my_input: Choices;
        match (line.chars().nth(2).unwrap(), opponent_input) {
            ('X', Choices::Paper) | ('Y', Choices::Rock) | ('Z', Choices::Scissors) => my_input = Choices::Rock,
            ('X', Choices::Scissors) | ('Y', Choices::Paper) | ('Z', Choices::Rock)  => my_input = Choices::Paper,
            ('X', Choices::Rock) | ('Y', Choices::Scissors) | ('Z', Choices::Paper)  => my_input = Choices::Scissors,
            _ => my_input = Choices::None,
        }

        // Calculate shape points
        match my_input {
            Choices::Rock => round_points += 1,
            Choices::Paper => round_points += 2,
            Choices::Scissors => round_points += 3,
            _ => (),
        }

        // Calculate round points
        match line.chars().nth(2).unwrap() {
            // I win
            'Z' => round_points += 6,

            // I lose
            'X' => round_points += 0,

            // Draw
            _ => round_points += 3,
        }

        final_score += round_points;
    }

    println!("Second: {final_score}")
}
