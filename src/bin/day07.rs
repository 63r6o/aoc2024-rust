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
                        .split(" ")
                        .map(|num_text| num_text.parse().unwrap())
                        .collect();

                    (value, equation_nums)
                })
                .unwrap()
        })
        .collect();

    fn is_valid(
        expected_result: &i64,
        nums: &Vec<i64>,
        current_result: i64,
        with_concat: bool,
    ) -> bool {
        if nums.len() == 0 {
            return *expected_result == current_result;
        }

        if expected_result < &current_result {
            return false;
        }

        let current_result_add = if current_result == -1 {
            0 + nums[0]
        } else {
            current_result + nums[0]
        };
        let current_result_mul = if current_result == -1 {
            1 * nums[0]
        } else {
            current_result * nums[0]
        };
        let current_result_con = if current_result == -1 {
            nums[0]
        } else {
            let concated = current_result.to_string() + &nums[0].to_string();
            concated.parse().unwrap()
        };

        return is_valid(
            expected_result,
            &nums[1..].to_vec(),
            current_result_add,
            with_concat,
        ) || is_valid(
            expected_result,
            &nums[1..].to_vec(),
            current_result_mul,
            with_concat,
        ) || (with_concat
            && is_valid(
                expected_result,
                &nums[1..].to_vec(),
                current_result_con,
                with_concat,
            ));
    }

    let part_one: i64 = equations
        .iter()
        .filter_map(|(expected_result, nums)| {
            if is_valid(expected_result, nums, -1, false) {
                Some(expected_result)
            } else {
                None
            }
        })
        .sum();

    let part_two: i64 = equations
        .iter()
        .filter_map(|(expected_result, nums)| {
            if is_valid(expected_result, nums, -1, true) {
                Some(expected_result)
            } else {
                None
            }
        })
        .sum();


    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
