use std::collections::HashSet;

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let trail_map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.to_digit(10).map(|d| d as i32))
                .collect()
        })
        .collect();

    fn traverse_dfs(
        prev: i32,
        (row, col): (usize, usize),
        trail_map: &Vec<Vec<i32>>,
        seen: &mut HashSet<(usize, usize)>,
        rating: bool,
    ) -> i32 {
        if trail_map.len() <= row
            || trail_map[0].len() <= col
            || trail_map[row][col] - prev != 1
            || seen.contains(&(row, col))
        {
            return 0;
        }
        if !rating {
            seen.insert((row, col));
        }

        if trail_map[row][col] == 9 {
            return 1;
        }

        traverse_dfs(trail_map[row][col], (row + 1, col), trail_map, seen, rating)
            + traverse_dfs(trail_map[row][col], (row, col + 1), trail_map, seen, rating)
            + if row > 0 {
                traverse_dfs(trail_map[row][col], (row - 1, col), trail_map, seen, rating)
            } else {
                0
            }
            + if col > 0 {
                traverse_dfs(trail_map[row][col], (row, col - 1), trail_map, seen, rating)
            } else {
                0
            }
    }

    let mut part_one = 0;
    let mut part_two = 0;
    trail_map.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, pos)| {
            if *pos == 0 {
                let mut seen = HashSet::new();
                part_two += traverse_dfs(-1, (row, col), &trail_map, &mut seen, true);
                part_one += traverse_dfs(-1, (row, col), &trail_map, &mut seen, false);
            }
        })
    });

    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
