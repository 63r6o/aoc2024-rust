use std::collections::VecDeque;

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let equations: Vec<(i64, Vec<i64>)> = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(value_text, equation_text)| {
                    let value: i64 = value_text.parse().unwrap();
                    let equation_nums: Vec<i64> = equation_text
                        .split(' ')
                        .map(|num_text| num_text.parse().unwrap())
                        .collect();

                    (value, equation_nums)
                })
                .unwrap()
        })
        .collect();

    fn is_valid(expected_result: i64, nums: &[i64], with_concat: bool) -> bool {
        let mut queue: VecDeque<(usize, i64)> = VecDeque::new();
        queue.push_back((nums.len() - 1, expected_result));

        while !queue.is_empty() {
            let (curr_index, curr_result) = queue.pop_front().unwrap();

            if curr_index == 0 && (curr_result == 1 || curr_result == nums[0]) {
                return true;
            }

            if curr_result % nums[curr_index] == 0 && 0 < curr_index {
                queue.push_back((curr_index - 1, curr_result / nums[curr_index]))
            }

            let curr_result_string = curr_result.to_string();
            let num_string = nums[curr_index].to_string();
            if with_concat && curr_result_string.ends_with(&num_string) && 0 < curr_index {
                let deconcatenated_result = &curr_result_string
                    [..curr_result_string.len() - num_string.len()]
                    .parse::<i64>()
                    .unwrap_or_default();
                queue.push_back((curr_index - 1, *deconcatenated_result))
            }

            if curr_result - nums[curr_index] > 0 && 0 < curr_index {
                queue.push_back((curr_index - 1, curr_result - nums[curr_index]))
            }
        }

        false
    }

    let (part_one, part_two) =
        equations
            .iter()
            .fold((0, 0), |(curr_one, curr_two), (expected_result, nums)| {
                let new_one = if is_valid(*expected_result, nums, false) {
                    curr_one + expected_result
                } else {
                    curr_one
                };

                let new_two = if new_one != curr_one || is_valid(*expected_result, nums, true) {
                    curr_two + expected_result
                } else {
                    curr_two
                };

                (new_one, new_two)
            });

    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
