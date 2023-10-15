//use std::env;
use std::fs;

fn main() {
    let file_path = "/Users/mateo.covacho/programming/Advent_of_code/Day_1/src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let calories_array = contents.split("\n\n");

    let mut calorie_value_vector: Vec<i64> = vec![];
    let mut i: i64 = 0;

    for line in calories_array {
        i = i + 1;
        let values = line.split("\n");
        let mut elf_value: i64 = 0;

        for num_str in values {
            elf_value += num_str.parse::<i64>().unwrap_or(0);
        }

        //print!("{}\n", elf_value.to_string());

        calorie_value_vector.push(elf_value);
    }
    print!("{}", calorie_value_vector.iter().max().unwrap().to_string());
}
