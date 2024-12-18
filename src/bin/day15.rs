use std::collections::HashMap;

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let (map, moves) = input
        .split_once("\n\n")
        .map(|(map, moves)| {
            (
                map.lines()
                    .map(|line| line.chars().collect())
                    .collect::<Vec<Vec<char>>>(),
                moves
                    .split('\n')
                    .collect::<Vec<&str>>()
                    .join("")
                    .chars()
                    .collect::<Vec<char>>(),
            )
        })
        .unwrap();

    let (mut robot_row, mut robot_col) = map
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|ch| ch == &'@').map(|col| (row, col)))
        .unwrap();

    let mut part_one_map = map.clone();

    let mut move_robot = |(row_delta, col_delta): (i32, i32)| {
        let (row, col) = (
            (robot_row as i32 + row_delta) as usize,
            (robot_col as i32 + col_delta) as usize,
        );

        match part_one_map[row][col] {
            'O' => {
                let (mut cur_row, mut cur_col) = (row, col);
                while part_one_map[cur_row][cur_col] != '#' {
                    if part_one_map[cur_row][cur_col] == '.' {
                        part_one_map[cur_row][cur_col] = 'O';

                        part_one_map[robot_row][robot_col] = '.';
                        robot_row = row;
                        robot_col = col;
                        part_one_map[robot_row][robot_col] = '@';
                        break;
                    }
                    part_one_map[cur_row][cur_col] = 'O';
                    cur_row = (cur_row as i32 + row_delta) as usize;
                    cur_col = (cur_col as i32 + col_delta) as usize;
                }
            }
            '.' => {
                part_one_map[robot_row][robot_col] = '.';
                robot_row = row;
                robot_col = col;
                part_one_map[robot_row][robot_col] = '@';
            }
            _ => (),
        }
    };

    moves.iter().for_each(|dir| match dir {
        '>' => move_robot((0, 1)),
        '<' => move_robot((0, -1)),
        '^' => move_robot((-1, 0)),
        'v' => move_robot((1, 0)),
        _ => panic!("Invalid move!"),
    });

    let part_one = part_one_map
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, ch)| {
                    if ch == &'O' {
                        Some((row * 100) + col)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("part one: {part_one}");

    let mut part_two_map = map
        .iter()
        .map(|line| {
            line.iter()
                .fold(vec![], |mut wider, ch| {
                    let new_ch = match ch {
                        '#' => "##",
                        'O' => "[]",
                        '.' => "..",
                        '@' => "@.",
                        _ => panic!("Invalid map"),
                    };

                    wider.push(new_ch);
                    wider
                })
                .join("")
                .chars()
                .collect()
        })
        .collect::<Vec<Vec<char>>>();

    (robot_row, robot_col) = part_two_map
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|ch| *ch == '@').map(|col| (row, col)))
        .unwrap();

    fn move_robot_but_wider(
        (row_delta, col_delta): (i32, i32),
        (curr_row, curr_col): (usize, usize),
        map: &Vec<Vec<char>>,
        boxes_to_move: &mut HashMap<(usize, usize), char>,
    ) -> bool {
        if boxes_to_move.contains_key(&(curr_row, curr_col)) {
            return true;
        }

        if map[curr_row][curr_col] == '.' {
            return true;
        }

        if map[curr_row][curr_col] == '#' {
            return false;
        }

        boxes_to_move.insert((curr_row, curr_col), map[curr_row][curr_col]);

        let is_left = map[curr_row][curr_col] == '[';
        let (prev_row, prev_col) = (curr_row, curr_col);

        let (curr_row, curr_col) = (
            (curr_row as i32 + row_delta) as usize,
            (curr_col as i32 + col_delta) as usize,
        );

        move_robot_but_wider(
            (row_delta, col_delta),
            (curr_row, curr_col),
            map,
            boxes_to_move,
        ) && if is_left {
            move_robot_but_wider(
                (row_delta, col_delta),
                (prev_row, prev_col + 1),
                map,
                boxes_to_move,
            )
        } else {
            move_robot_but_wider(
                (row_delta, col_delta),
                (prev_row, prev_col - 1),
                map,
                boxes_to_move,
            )
        }
    }

    fn update_map(
        map: &mut [Vec<char>],
        boxes_to_move: &HashMap<(usize, usize), char>,
        (row_delta, col_delta): (i32, i32),
    ) {
        for (row, col) in boxes_to_move.keys() {
            map[*row][*col] = '.'
        }

        for ((row, col), ch) in boxes_to_move {
            let (new_row, new_col) = (
                (*row as i32 + row_delta) as usize,
                (*col as i32 + col_delta) as usize,
            );
            map[new_row][new_col] = *ch
        }
    }

    moves.iter().for_each(|dir| {
        let mut boxes_to_move = HashMap::new();
        boxes_to_move.insert((robot_row, robot_col), '@');
        match dir {
            '>' => {
                let can_be_moved = move_robot_but_wider(
                    (0, 1),
                    (robot_row, robot_col + 1),
                    &part_two_map,
                    &mut boxes_to_move,
                );

                if can_be_moved {
                    robot_col += 1;

                    update_map(&mut part_two_map, &boxes_to_move, (0, 1));
                }
            }
            '<' => {
                let can_be_moved = move_robot_but_wider(
                    (0, -1),
                    (robot_row, robot_col - 1),
                    &part_two_map,
                    &mut boxes_to_move,
                );

                if can_be_moved {
                    robot_col -= 1;

                    update_map(&mut part_two_map, &boxes_to_move, (0, -1));
                }
            }
            '^' => {
                let can_be_moved = move_robot_but_wider(
                    (-1, 0),
                    (robot_row - 1, robot_col),
                    &part_two_map,
                    &mut boxes_to_move,
                );

                if can_be_moved {
                    robot_row -= 1;

                    update_map(&mut part_two_map, &boxes_to_move, (-1, 0));
                }
            }
            'v' => {
                let can_be_moved = move_robot_but_wider(
                    (1, 0),
                    (robot_row + 1, robot_col),
                    &part_two_map,
                    &mut boxes_to_move,
                );

                if can_be_moved {
                    robot_row += 1;

                    update_map(&mut part_two_map, &boxes_to_move, (1, 0));
                }
            }
            _ => panic!("Invalid move!"),
        }
    });

    let part_two = part_two_map
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, ch)| {
                    if ch == &'[' {
                        Some((row * 100) + col)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("part two: {part_two}");
}
