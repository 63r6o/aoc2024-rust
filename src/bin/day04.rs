use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let char_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let chars_to_search = vec!['X', 'M', 'A', 'S'];

    enum Direction {
        Up,
        Down,
        Left,
        Right,
        UpLeft,
        UpRight,
        DownLeft,
        DownRight,
    }

    let xmases = |row: usize, col: usize, direction: Direction| {
        for index in 0..chars_to_search.len() {
            let mut new_row = row;
            let mut new_col = col;
            match direction {
                Direction::Up => {
                    if row.checked_sub(index).is_none() {
                        return 0;
                    }
                    new_row -= index
                }
                Direction::Down => {
                    if row + index >= char_map.len() {
                        return 0;
                    }
                    new_row += index
                }
                Direction::Left => {
                    if col.checked_sub(index).is_none() {
                        return 0;
                    }
                    new_col -= index
                }
                Direction::Right => {
                    if col + index >= char_map[0].len() {
                        return 0;
                    }
                    new_col += index
                }
                Direction::UpLeft => {
                    if row.checked_sub(index).is_none() || col.checked_sub(index).is_none() {
                        return 0;
                    }
                    new_row -= index;
                    new_col -= index
                }
                Direction::UpRight => {
                    if row.checked_sub(index).is_none() || col + index >= char_map[0].len() {
                        return 0;
                    }
                    new_row -= index;
                    new_col += index
                }
                Direction::DownLeft => {
                    if row + index >= char_map.len() || col.checked_sub(index).is_none() {
                        return 0;
                    }
                    new_row += index;
                    new_col -= index
                }
                Direction::DownRight => {
                    if row + index >= char_map.len() || col + index >= char_map[0].len() {
                        return 0;
                    }
                    new_row += index;
                    new_col += index
                }
            }
            if char_map[new_row][new_col] != chars_to_search[index] {
                return 0;
            }
        }

        return 1;
    };

    let is_xmas = |row: usize, col: usize| {
        if char_map[row][col] != 'A'
            || row < 1
            || char_map.len() <= row + 1
            || col < 1
            || char_map[0].len() <= col + 1
        {
            return 0;
        }

        let is_left_ok = (char_map[row - 1][col - 1] == 'M' && char_map[row + 1][col + 1] == 'S')
            || (char_map[row - 1][col - 1] == 'S' && char_map[row + 1][col + 1] == 'M');

        let is_right_ok = (char_map[row + 1][col - 1] == 'M' && char_map[row - 1][col + 1] == 'S')
            || (char_map[row + 1][col - 1] == 'S' && char_map[row - 1][col + 1] == 'M');

        if is_left_ok && is_right_ok {
            return 1;
        } else {
            return 0;
        }
    };

    let mut part_one = 0;
    let mut part_two = 0;
    for (row, line) in char_map.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            part_one += xmases(row, col, Direction::Up)
                + xmases(row, col, Direction::Down)
                + xmases(row, col, Direction::Left)
                + xmases(row, col, Direction::Right)
                + xmases(row, col, Direction::UpLeft)
                + xmases(row, col, Direction::UpRight)
                + xmases(row, col, Direction::DownLeft)
                + xmases(row, col, Direction::DownRight);

            part_two += is_xmas(row, col)
        }
    }

    println!("part one: {part_one}");
    println!("part two: {part_two}");
}
