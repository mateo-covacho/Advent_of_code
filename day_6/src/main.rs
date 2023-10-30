use std::{collections::HashSet, fs};

pub fn main(s: Option<String>) {
    let contents = s.unwrap_or(fs::read_to_string("src/input.txt").unwrap());

    let mut count = 0;
    let mut chars_vec: Vec<char> = Vec::with_capacity(4);
    for curr_char in contents.chars() {
        if chars_vec.len() >= 4 {
            chars_vec.remove(0);
        };

        count += 1;
        chars_vec.push(curr_char);

        // check if is valid signal

        let mut has_duplicates = false;

        let mut seen = HashSet::new();

        for item in &chars_vec {
            if seen.contains(&item) {
                has_duplicates = true;
            }
            seen.insert(item);
        }

        if has_duplicates {
            //println!("{:?} has duplicates {}", chars_vec, has_duplicates)
        };

        if !has_duplicates && chars_vec.len() == 4 {
            break;
        }
    }
    print!("{}", count);
    print!("{:?}", chars_vec);
}

pub fn find_ 