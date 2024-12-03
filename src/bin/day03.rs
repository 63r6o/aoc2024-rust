use aoc2024_rust::read_from_args;
use regex::Regex;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let numbers_regex = Regex::new(r"\d+").unwrap();
    let instructions_regex = Regex::new(r"don't\(\)|do\(\)|mul\(\d+,\d+\)").unwrap();

    let instructions: Vec<&str> = instructions_regex
        .find_iter(&input)
        .map(|mat| &input[mat.start()..mat.end()])
        .collect();

    let mut part_one = 0;
    let mut part_two = 0;
    let mut enabled = true;

    for instruction in instructions {
        match instruction {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let prod = numbers_regex
                    .find_iter(instruction)
                    .map(|mat| {
                        (&instruction[mat.start()..mat.end()])
                            .parse::<i32>()
                            .unwrap()
                    })
                    .fold(1, |prod, num| prod * num);

                if enabled {
                    part_two += prod
                }
                part_one += prod
            }
        }
    }

    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
