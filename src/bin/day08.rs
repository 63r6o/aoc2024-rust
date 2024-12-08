use std::{
    collections::{HashMap, HashSet},
    usize,
};

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            if ch == '.' {
                return;
            }

            let coordinates = antennas.entry(ch).or_default();
            coordinates.push((row, col))
        })
    });

    let col_length = input.find('\n').unwrap();
    let get_antinodes_position = |coords: (usize, usize), other_coords: (usize, usize)| {
        let (x1, y1) = (coords.0 as i32, coords.1 as i32);
        let (x2, y2) = (other_coords.0 as i32, other_coords.1 as i32);

        let row_diff = x1.abs_diff(x2) as i32;
        let col_diff = y1.abs_diff(y2) as i32;

        let x3 = if x1 < x2 {
            x1 - row_diff
        } else {
            x1 + row_diff
        };

        let y3 = if y1 < y2 {
            y1 - col_diff
        } else {
            y1 + col_diff
        };

        let x4 = if x1 < x2 {
            x2 + row_diff
        } else {
            x2 - row_diff
        };

        let y4 = if y1 < y2 {
            y2 + col_diff
        } else {
            y2 - col_diff
        };

        let first =
            if 0 <= x3 && (x3 as usize) < col_length && 0 <= y3 && (y3 as usize) < col_length {
                Some((x3 as usize, y3 as usize))
            } else {
                None
            };

        let second =
            if 0 <= x4 && (x4 as usize) < col_length && 0 <= y4 && (y4 as usize) < col_length {
                Some((x4 as usize, y4 as usize))
            } else {
                None
            };

        (first, second)
    };

    let mut antinode_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut antinode_positions_two: HashSet<(usize, usize)> = HashSet::new();
    antennas.values().for_each(|positions_vec| {
        if positions_vec.len() > 1 {
            positions_vec.iter().for_each(|coord| {
                antinode_positions_two.insert(*coord);
            })
        };

        for i in 0..(positions_vec.len() - 1) {
            for j in (i + 1)..positions_vec.len() {
                let (first, second) = get_antinodes_position(positions_vec[i], positions_vec[j]);
                if let Some(coords) = first {
                    antinode_positions.insert(coords);

                    let mut curr_second = positions_vec[i];
                    let mut curr_first = first;
                    while curr_first.is_some() {
                        let tmp = curr_first.unwrap();
                        antinode_positions_two.insert(tmp);
                        (curr_first, _) = get_antinodes_position(curr_first.unwrap(), curr_second);
                        curr_second = tmp;
                    }
                }

                if let Some(coords) = second {
                    antinode_positions.insert(coords);

                    let mut curr_first = positions_vec[j];
                    let mut curr_second = second;
                    while curr_second.is_some() {
                        let tmp = curr_second.unwrap();
                        antinode_positions_two.insert(tmp);
                        (_, curr_second) = get_antinodes_position(curr_first, curr_second.unwrap());
                        curr_first = tmp;
                    }
                }
            }
        }
    });

    let part_one = antinode_positions.len();
    let part_two = antinode_positions_two.len();

    println!("part one: {part_one}");
    println!("part two: {part_two}");
}
