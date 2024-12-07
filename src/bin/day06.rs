use std::collections::HashSet;

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let (mut start_row, mut start_col) = (0, 0);
    let mut lab_map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            if let Some(col) = line.find("^") {
                start_row = row;
                start_col = col;
            };
            line.chars().collect()
        })
        .collect::<Vec<Vec<char>>>();

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn patrol(
        positions: &mut HashSet<(usize, usize)>,
        current_row: usize,
        current_col: usize,
        direction: Direction,
        lab_map: &mut Vec<Vec<char>>,
    ) -> usize {
        positions.insert((current_row, current_col));

        let obstacle_ahead = match direction {
            Direction::Up => {
                if current_row < 1 {
                    return positions.len();
                }

                lab_map[current_row - 1][current_col] == '#'
            }
            Direction::Down => {
                if lab_map.len() - 1 <= current_row {
                    return positions.len();
                }

                lab_map[current_row + 1][current_col] == '#'
            }
            Direction::Left => {
                if current_col < 1 {
                    return positions.len();
                }

                lab_map[current_row][current_col - 1] == '#'
            }
            Direction::Right => {
                if lab_map[0].len() - 1 <= current_col {
                    return positions.len();
                }

                lab_map[current_row][current_col + 1] == '#'
            }
        };

        let new_direction = if !obstacle_ahead {
            direction
        } else {
            match direction {
                Direction::Up => {
                    lab_map[current_row][current_col] = 'R';
                    Direction::Right
                }
                Direction::Down => {
                    lab_map[current_row][current_col] = 'L';
                    Direction::Left
                }
                Direction::Left => {
                    lab_map[current_row][current_col] = 'U';
                    Direction::Up
                }
                Direction::Right => {
                    lab_map[current_row][current_col] = 'D';
                    Direction::Down
                }
            }
        };

        let (new_row, new_col) = match new_direction {
            Direction::Up => (current_row - 1, current_col),
            Direction::Down => (current_row + 1, current_col),
            Direction::Left => (current_row, current_col - 1),
            Direction::Right => (current_row, current_col + 1),
        };

        patrol(positions, new_row, new_col, new_direction, lab_map)
    }

    let mut positions = HashSet::<(usize, usize)>::new();

    let part_one = patrol(
        &mut positions,
        start_row,
        start_col,
        Direction::Up,
        &mut lab_map,
    );

    println!("part one: {}", part_one);

    fn is_loop_patrol(
        positions: &mut HashSet<(usize, usize, &str)>,
        current_row: usize,
        current_col: usize,
        direction: Direction,
        lab_map: &Vec<Vec<char>>,
    ) -> i32 {
        let dir_string = match direction {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        };
        if !positions.insert((current_row, current_col, dir_string)) {
            return 1;
        }

        let obstacle_ahead = match direction {
            Direction::Up => {
                if current_row < 1 {
                    return 0;
                }

                lab_map[current_row - 1][current_col] == '#'
            }
            Direction::Down => {
                if lab_map.len() - 1 <= current_row {
                    return 0;
                }

                lab_map[current_row + 1][current_col] == '#'
            }
            Direction::Left => {
                if current_col < 1 {
                    return 0;
                }

                lab_map[current_row][current_col - 1] == '#'
            }
            Direction::Right => {
                if lab_map[0].len() - 1 <= current_col {
                    return 0;
                }

                lab_map[current_row][current_col + 1] == '#'
            }
        };

        let new_direction = if !obstacle_ahead {
            direction
        } else {
            match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        };

        let (new_row, new_col) = match new_direction {
            Direction::Up => (current_row - 1, current_col),
            Direction::Down => (current_row + 1, current_col),
            Direction::Left => (current_row, current_col - 1),
            Direction::Right => (current_row, current_col + 1),
        };

        is_loop_patrol(positions, new_row, new_col, new_direction, lab_map)
    }

    let part_two = positions.iter().fold(0, |res, (row, col)| {
        if row == &start_row && col == &start_col {
            return res;
        }

        let mut obstacled_map = lab_map.clone();
        obstacled_map[*row][*col] = '#';

        let mut positions = HashSet::new();

        res + is_loop_patrol(
            &mut positions,
            start_row,
            start_col,
            Direction::Up,
            &obstacled_map,
        )
    });
    println!("part two: {part_two}")
}
