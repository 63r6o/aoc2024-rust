use std::collections::HashSet;

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    #[derive(Debug)]
    struct Perimeters {
        up: Vec<(usize, usize)>,
        left: Vec<(usize, usize)>,
        rigth: Vec<(usize, usize)>,
        down: Vec<(usize, usize)>,
    }

    fn get_area_and_set_perimeters(
        (row, col): (usize, usize),
        plant: &char,
        seen: &mut HashSet<(usize, usize)>,
        perimeters: &mut Perimeters,
        garden: &Vec<Vec<char>>,
    ) -> i32 {
        if seen.contains(&(row, col)) {
            return 0;
        }

        seen.insert((row, col));

        let mut area = 1;
        if row == 0 || garden[row - 1][col] != *plant {
            perimeters.up.push((row, col));
        }
        if row == garden.len() - 1 || garden[row + 1][col] != *plant {
            perimeters.down.push((row, col));
        }
        if col == 0 || garden[row][col - 1] != *plant {
            perimeters.left.push((row, col));
        }
        if col == garden[0].len() - 1 || garden[row][col + 1] != *plant {
            perimeters.rigth.push((row, col));
        }

        if row < garden.len() - 1 && garden[row + 1][col] == *plant {
            let down_area =
                get_area_and_set_perimeters((row + 1, col), plant, seen, perimeters, garden);
            area += down_area;
        }

        if row > 0 && garden[row - 1][col] == *plant {
            let up_area =
                get_area_and_set_perimeters((row - 1, col), plant, seen, perimeters, garden);
            area += up_area;
        }

        if col > 0 && garden[row][col - 1] == *plant {
            let left_area =
                get_area_and_set_perimeters((row, col - 1), plant, seen, perimeters, garden);
            area += left_area;
        }

        if col < garden[0].len() - 1 && garden[row][col + 1] == *plant {
            let right_area =
                get_area_and_set_perimeters((row, col + 1), plant, seen, perimeters, garden);
            area += right_area;
        }

        area
    }

    fn count_sides(sorted_perimeters: &[(usize, usize)], vertical: bool) -> i32 {
        if sorted_perimeters.is_empty() {
            return 0;
        }
        let (mut prev_row, mut prev_col) = sorted_perimeters[0];
        let mut res = 1;
        for (curr_row, curr_col) in sorted_perimeters.iter().skip(1) {
            if vertical && (*curr_row != prev_row + 1 || *curr_col != prev_col)
                || !vertical && (*curr_row != prev_row || *curr_col != prev_col + 1)
            {
                res += 1;
            }

            prev_row = *curr_row;
            prev_col = *curr_col;
        }
        res
    }

    let mut part_one = 0;
    let mut part_two = 0;

    let mut seen = HashSet::new();

    garden.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, plant)| {
            let mut perimeters = Perimeters {
                up: vec![],
                left: vec![],
                rigth: vec![],
                down: vec![],
            };

            let area =
                get_area_and_set_perimeters((row, col), plant, &mut seen, &mut perimeters, &garden);

            part_one += area
                * (perimeters.up.len()
                    + perimeters.down.len()
                    + perimeters.left.len()
                    + perimeters.rigth.len()) as i32;

            perimeters.up.sort_by(|(a_row, a_col), (b_row, b_col)| {
                if a_row == b_row {
                    a_col.cmp(b_col)
                } else {
                    a_row.cmp(b_row)
                }
            });
            perimeters.down.sort_by(|(a_row, a_col), (b_row, b_col)| {
                if a_row == b_row {
                    a_col.cmp(b_col)
                } else {
                    a_row.cmp(b_row)
                }
            });
            perimeters.left.sort_by(|(a_row, a_col), (b_row, b_col)| {
                if a_col == b_col {
                    a_row.cmp(b_row)
                } else {
                    a_col.cmp(b_col)
                }
            });
            perimeters.rigth.sort_by(|(a_row, a_col), (b_row, b_col)| {
                if a_col == b_col {
                    a_row.cmp(b_row)
                } else {
                    a_col.cmp(b_col)
                }
            });

            let mut sides = 0;
            sides += count_sides(&perimeters.up, false);
            sides += count_sides(&perimeters.down, false);
            sides += count_sides(&perimeters.left, true);
            sides += count_sides(&perimeters.rigth, true);

            part_two += area * sides;
        })
    });

    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
