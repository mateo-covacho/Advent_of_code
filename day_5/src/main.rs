use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
fn main() {
    let contents = fs::read_to_string("src/input.txt").unwrap();

    let is_problem_2 = false;

    let (crates_str, commands_str) = contents.split_once("\n\n").unwrap();

    // Parse commands
    let commands: Vec<Command> = commands_str
        .split('\n')
        .into_iter()
        .map(|line| {
            let line_elements: Vec<&str> = line.split_whitespace().collect();
            //print!("{:?}\n", line_elements);
            Command {
                quantity: line_elements[1].parse().unwrap_or(0),
                from: line_elements[3].parse().unwrap_or(0),
                to: line_elements[5].parse().unwrap_or(0),
            }
        })
        .collect();

    // Parse crates
    let mut crates: HashMap<i32, Vec<char>> = HashMap::new();
    crates.insert(1, vec!['R', 'N', 'F', 'V', 'L', 'J', 'S', 'M']);
    crates.insert(2, vec!['P', 'N', 'D', 'Z', 'F', 'J', 'W', 'H']);
    crates.insert(3, vec!['W', 'R', 'C', 'D', 'G']);
    crates.insert(4, vec!['N', 'B', 'S']);
    crates.insert(5, vec!['M', 'Z', 'W', 'P', 'C', 'B', 'F', 'N']);
    crates.insert(6, vec!['P', 'R', 'M', 'W']);
    crates.insert(7, vec!['R', 'T', 'N', 'G', 'L', 'S', 'W']);
    crates.insert(8, vec!['Q', 'T', 'H', 'F', 'N', 'B', 'V']);
    crates.insert(9, vec!['Q', 'T', 'H', 'F', 'N', 'B', 'V']);

    pretty_print_sorted(&crates);
    println!("_________________________");

    for command in commands {
        if let Some(origin_crates_vec) = crates.get_mut(&command.from) {
            let moved_crates =
                origin_crates_vec.split_off(origin_crates_vec.len() - command.quantity as usize);

            let mut reversed_crates = moved_crates;

            if (is_problem_2) {
                reversed_crates.reverse();
            }
            if let Some(target_crates_vec) = crates.get_mut(&command.to) {
                target_crates_vec.append(&mut reversed_crates);
            }
        }
        //println!("{} from {} to {}",command.quantity, command.from, command.to);
    }
    println!("");
    pretty_print_sorted(&crates);
}

#[derive(Debug)]
struct Command {
    quantity: i32,
    from: i32,
    to: i32,
}

fn pretty_print_sorted(map: &HashMap<i32, Vec<char>>) {
    let mut keys: Vec<&i32> = map.keys().collect();
    keys.sort();

    for key in keys {
        if let Some(value) = map.get(key) {
            print!("{}: [", key);
            for (i, c) in value.iter().enumerate() {
                if i != 0 {
                    print!(", ");
                }
                print!("{}", c);
            }
            println!("]");
        }
    }
}
// Part 1 BQDNWJPVJ
// Part 2 QPJPLMNNR
