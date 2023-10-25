mod main;

use main::calculate_char_value;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut chars: Vec<char> = vec![];
    for line in lines.chunks(3) {
        let compartments = line.split_at(line.len() / 2);

        let mut duplicate_char = '\0';
        for c in compartments.0.chars() {
            if compartments.1.find(c).is_some() {
                duplicate_char = c
            }
        }

        chars.push(duplicate_char);
    }

    println!(
        "{}",
        chars.iter().map(|x| calculate_char_value(*x)).sum::<i32>()
    )
}
