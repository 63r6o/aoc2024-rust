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

    fn is_valid(
        expected_result: &i64,
        nums: &[i64],
        current_result: i64,
        with_concat: bool,
    ) -> bool {
        if nums.is_empty() {
            return *expected_result == current_result;
        }

        if expected_result < &current_result {
            return false;
        }

        let current_result_add = if current_result == -1 {
            nums[0]
        } else {
            current_result + nums[0]
        };
        let current_result_mul = if current_result == -1 {
            nums[0]
        } else {
            current_result * nums[0]
        };
        let current_result_con = if current_result == -1 {
            nums[0]
        } else {
            // https://www.reddit.com/r/rust/comments/191l3ot/concatinate_two_numbers/
            current_result * 10_i64.pow(nums[0].ilog10() + 1) + nums[0]
            // let concated = current_result.to_string() + &nums[0].to_string();
            // concated.parse().unwrap()
        };

        is_valid(expected_result, &nums[1..], current_result_add, with_concat)
            || is_valid(expected_result, &nums[1..], current_result_mul, with_concat)
            || (with_concat
                && is_valid(expected_result, &nums[1..], current_result_con, with_concat))
    }

    let (part_one, part_two) =
        equations
            .iter()
            .fold((0, 0), |(curr_one, curr_two), (expected_result, nums)| {
                let new_one = if is_valid(expected_result, nums, -1, false) {
                    curr_one + expected_result
                } else {
                    curr_one
                };
                let new_two = if is_valid(expected_result, nums, -1, true) {
                    curr_two + expected_result
                } else {
                    curr_two
                };

                (new_one, new_two)
            });

    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
