use std::collections::HashMap;

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let initial_stones: Vec<i64> = input
        .split(' ')
        .filter_map(|num_slice| num_slice.trim().parse().ok())
        .collect();

    fn number_of_child_stones(
        stone: i64,
        remaining_blinks: i32,
        seen: &mut HashMap<(i64, i32), i64>,
    ) -> i64 {
        if remaining_blinks == 0 {
            return 1;
        }

        if seen.contains_key(&(stone, remaining_blinks)) {
            return *seen.get(&(stone, remaining_blinks)).unwrap();
        }

        let stone_as_string = stone.to_string();
        let children = if stone == 0 {
            number_of_child_stones(1, remaining_blinks - 1, seen)
        } else if stone_as_string.len() % 2 == 0 {
            let (left, right) = stone_as_string.split_at(stone_as_string.len() / 2);

            number_of_child_stones(left.parse().unwrap(), remaining_blinks - 1, seen)
                + number_of_child_stones(right.parse().unwrap(), remaining_blinks - 1, seen)
        } else {
            number_of_child_stones(stone * 2024, remaining_blinks - 1, seen)
        };

        seen.insert((stone, remaining_blinks), children);
        children
    }

    let mut part_one = 0;
    let mut part_two = 0;
    let mut seen: HashMap<(i64, i32), i64> = HashMap::new();
    initial_stones.iter().for_each(|stone| {
        part_one += number_of_child_stones(*stone, 25, &mut seen);
        part_two += number_of_child_stones(*stone, 75, &mut seen);
    });
    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
