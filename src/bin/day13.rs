use std::collections::{BinaryHeap, HashSet};

use aoc2024_rust::read_from_args;
use regex::Regex;

#[derive(Debug)]
struct Position {
    used_tokens: i64,
    coords : (i64, i64),
    a_steps: i64,
    b_steps: i64
}

impl Eq for Position {}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.used_tokens == other.used_tokens
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.used_tokens.cmp(&other.used_tokens).reverse()
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let digits_regex = Regex::new(r"\d+").unwrap();

    let claws: Vec<Vec<(i64, i64)>> = input
        .split("\n\n")
        .map(|claw| {
            claw.lines()
                .map(|line| {
                    digits_regex
                        .find_iter(line)
                        .filter_map(|digit| digit.as_str().parse().ok())
                        .collect::<Vec<i64>>()
                })
                .map(|coords| match &coords[..] {
                    &[x, y, ..] => (x, y),
                    _ => panic!("Error in parsing the input!"),
                })
                .collect()
        })
        .collect();

    // my original solution for part 1
    let dijstra_like_stuff = |(x_a, y_a): (i64, i64), (x_b, y_b): (i64, i64), (x_prize, y_prize): (i64, i64)| -> (i64, i64) {
        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let mut min_heap: BinaryHeap<Position> = BinaryHeap::new();

        let starting_position = Position {
            used_tokens: 0,
            coords: (0, 0),
            a_steps: 0,
            b_steps: 0,
        };

        min_heap.push(starting_position);

        while !min_heap.is_empty() {
            let current_position = min_heap.pop().unwrap();

            if seen.contains(&current_position.coords) || current_position.a_steps > 100 || current_position.b_steps > 100 {
                continue;
            }

            seen.insert(current_position.coords);

            if current_position.coords == (x_prize, y_prize) {
                return (current_position.a_steps, current_position.b_steps)
            }

            // 3 tokens for A 1 tokens for B
            if current_position.coords.0 + x_a <= x_prize && current_position.coords.1 + y_a <= y_prize {
                min_heap.push(Position {
                    used_tokens: current_position.used_tokens + 3,
                    coords: (current_position.coords.0 + x_a, current_position.coords.1 + y_a),
                    a_steps: current_position.a_steps + 1,
                    b_steps: current_position.b_steps
                })
            }

            if current_position.coords.0 + x_b <= x_prize && current_position.coords.1 + y_b <= y_prize {
                min_heap.push(Position {
                    used_tokens: current_position.used_tokens + 1,
                    coords: (current_position.coords.0 + x_b, current_position.coords.1 + y_b),
                    a_steps: current_position.a_steps,
                    b_steps: current_position.b_steps + 1
                })
            }
        }

        (0, 0)
    };

    // For part 2, I couldn't figure it out by myself
    // https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
    let solve_it_with_maths = |(x_a, y_a): (i64, i64), (x_b, y_b): (i64, i64), (mut x_prize, mut y_prize): (i64, i64)| -> (i64, i64) {
        x_prize = x_prize + 10000000000000;
        y_prize = y_prize + 10000000000000;

        let a = (x_prize * y_b - y_prize * x_b) / (x_a * y_b - y_a * x_b);
        let b = (x_a * y_prize - y_a * x_prize) / (x_a * y_b - y_a * x_b);

        if (a * x_a + b * x_b, a * y_a + b * y_b) == (x_prize, y_prize) {
            return (a, b)
        }

        (0, 0)
    };

    let mut part_one = 0;
    let mut part_two = 0;
    claws.iter().for_each(|coords| {
        let (a_steps_one, b_steps_one) = dijstra_like_stuff(coords[0], coords[1], coords[2]);
        let (a_steps_two, b_steps_two) = solve_it_with_maths(coords[0], coords[1], (coords[2].0 as i64, coords[2].1 as i64));

        part_one += (a_steps_one * 3) + b_steps_one;
        part_two += (a_steps_two * 3) + b_steps_two;
    });

    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
