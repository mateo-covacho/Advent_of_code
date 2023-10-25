use std::fs;

#[derive(Debug)]
struct Squad {
    start: i32,
    end: i32,
}

fn main() {
    part_2();
}

fn part_1() {
    let contents =
        fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    let mut count: i64 = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();

        let first_squad_values: Vec<i32> = parts[0]
            .split('-')
            .map(|x| x.parse().unwrap_or(0))
            .collect();
        let squad_1: Squad = {
            Squad {
                start: first_squad_values[0],
                end: first_squad_values[1],
            }
        };

        let second_squad_values: Vec<i32> = parts[1]
            .split('-')
            .map(|x| x.parse().unwrap_or(0))
            .collect();
        let squad_2: Squad = {
            Squad {
                start: second_squad_values[0],
                end: second_squad_values[1],
            }
        };

        if (squad_1.start <= squad_2.start && squad_1.end >= squad_2.end)// 2 in 1
            || (squad_1.start >= squad_2.start && squad_1.end <= squad_2.end)
        // 1 in 2
        {
            count += 1;
        }
    }
    print!("{}", count)
}

fn part_2() {
    println!(
        "{}",
        include_str!("input1.txt")
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(',').unwrap();
                let ((s1_start, s1_end), (s2_start, s2_end)) =
                    (l.split_once('-').unwrap(), r.split_once('-').unwrap());
                (
                    s1_start.parse::<u8>().unwrap(),
                    s1_end.parse::<u8>().unwrap(),
                    s2_start.parse::<u8>().unwrap(),
                    s2_end.parse::<u8>().unwrap(),
                )
            })
            .filter(
                |(s1_start, s1_end, s2_start, s2_end)| 
				(s1_start >= s2_start && s1_end <= s2_end)
                    || (s1_start <= s2_start && s1_end >= s2_end)
            )
            .count()
    );
}
