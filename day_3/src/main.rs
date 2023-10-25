use std::fs;

pub fn calculate_char_value(character: char) -> i32 {
    let char_value;

    if character.is_uppercase() {
        char_value = (character as u8) - 38;
    } else {
        char_value = (character as u8) - 96;
    }
    char_value.into()
}

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut chars: Vec<char> = vec![];
    for line in contents.lines() {
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
