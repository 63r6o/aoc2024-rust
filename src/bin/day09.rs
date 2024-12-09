use std::iter::repeat;

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    // (is_file, index, size)
    let mut blocks: Vec<(bool, usize, usize)> = input
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            let size = ch.to_digit(10).unwrap_or_default() as usize;

            (i % 2 == 0, i / 2, size)
        })
        .collect();

    // (is_file, index)
    let mut blocks_part_one = blocks.iter().fold(vec![], |mut res, (is_file, id, size)| {
        let mut curr_block: Vec<(bool, usize)> = vec![(*is_file, *id)]
            .into_iter()
            .flat_map(|block| repeat(block).take(*size))
            .collect();

        res.append(&mut curr_block);
        res
    });

    let mut left: usize = 0;
    let mut right: usize = blocks_part_one.len() - 1;
    while left < right {
        while !blocks_part_one[right].0 {
            right -= 1
        }

        if !blocks_part_one[left].0 {
            blocks_part_one[left] = blocks_part_one[right];
            blocks_part_one[right].0 = false
        }

        left += 1
    }

    let part_one: usize = blocks_part_one
        .iter()
        .enumerate()
        .filter_map(|(i, (is_file, id))| if *is_file { id.checked_mul(i) } else { None })
        .sum();

    println!("part one: {part_one}");

    let mut curr_file_i = blocks.len() - 1;
    while curr_file_i > 0 {
        while !blocks[curr_file_i].0 {
            curr_file_i -= 1
        }

        for i in 0..curr_file_i {
            if blocks[i].0 || blocks[i].2 < blocks[curr_file_i].2 {
                continue;
            }

            if blocks[i].2 == blocks[curr_file_i].2 {
                blocks[i] = blocks[curr_file_i];
                blocks[curr_file_i].0 = false;
                break;
            } else {
                let tmp = blocks[curr_file_i];
                blocks[curr_file_i].0 = false;

                blocks[i].2 -= tmp.2;
                blocks.insert(i, tmp);
                break;
            }
        }

        curr_file_i -= 1
    }

    let part_two: usize = blocks
        .iter()
        .fold(vec![], |mut res, (is_file, id, size)| {
            let mut curr_block: Vec<(bool, usize)> = vec![(*is_file, *id)]
                .into_iter()
                .flat_map(|block| repeat(block).take(*size))
                .collect();

            res.append(&mut curr_block);
            res
        })
        .iter()
        .enumerate()
        .filter_map(|(i, (is_file, id))| if *is_file { id.checked_mul(i) } else { None })
        .sum();

    println!("part two: {part_two}")
}
