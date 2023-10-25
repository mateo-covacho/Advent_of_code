use std::fs;

// Opponent
// A for Rock
// B for Paper,
// C for Scissors
// ME
// X for Rock
// Y for Paper,
// Z for Scissors

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
        let opponent_input;
        match line.chars().nth(0).unwrap() {
            'A' => opponent_input = Choices::Rock,
            'B' => opponent_input = Choices::Paper,
            'C' => opponent_input = Choices::Scissors,
            _ => opponent_input = Choices::None,
        }

        let my_input: Choices;
        match line.chars().nth(2).unwrap() {
            'X' => my_input = Choices::Rock,
            'Y' => my_input = Choices::Paper,
            'Z' => my_input = Choices::Scissors,
            _ => my_input = Choices::None,
        }

        let mut round_points: i64 = 0;

        match (&my_input, &opponent_input) {
            // I win
            (Choices::Rock, Choices::Scissors)
            | (Choices::Scissors, Choices::Paper)
            | (Choices::Paper, Choices::Rock) => round_points += 6,

            // I lose
            (Choices::Scissors, Choices::Rock)
            | (Choices::Paper, Choices::Scissors)
            | (Choices::Rock, Choices::Paper) => round_points += 0,

            // Draw
            _ => round_points += 3,
        }

        match my_input {
            Choices::Rock => round_points += 1,
            Choices::Paper => round_points += 2,
            Choices::Scissors => round_points += 3,
            _ => (),
        }

        final_score += round_points;
    }

    println!("{final_score}")
}
