use std::{cmp::Ordering, collections::HashMap};

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let (ordering_rules_raw, updates_raw) = input.split_once("\n\n").unwrap();

    let mut comes_after: HashMap<&str, Vec<&str>> = HashMap::new();
    ordering_rules_raw.lines().for_each(|line| {
        let (before, after) = line.split_once("|").unwrap();
        let current_afters = comes_after.entry(before).or_insert(vec![]);
        current_afters.push(after)
    });

    let (part_one, part_two) = updates_raw
        .lines()
        .map(|line| {
            let pages: Vec<&str> = line.split(",").collect();

            for (i, page) in pages.iter().enumerate().rev() {
                for after in &pages[..i] {
                    if !comes_after.contains_key(page) {
                        continue;
                    }

                    if comes_after[page].contains(after) {
                        let mut ordered_pages = pages.clone();
                        ordered_pages.sort_by(|a, b| {
                            if let Some(vec) = comes_after.get(a) {
                                if vec.contains(b) {
                                    return Ordering::Less;
                                }
                            }
                            if let Some(vec) = comes_after.get(b) {
                                if vec.contains(a) {
                                    return Ordering::Greater;
                                }
                            }

                            Ordering::Equal
                        });
                        return (
                            0,
                            ordered_pages[ordered_pages.len() / 2]
                                .parse::<i32>()
                                .unwrap(),
                        );
                    }
                }
            }
            return (pages[pages.len() / 2].parse::<i32>().unwrap(), 0);
        })
        .fold(
            (0, 0),
            |(sum_corrects, sum_incorrects), (correct, incorrect)| {
                (sum_corrects + correct, sum_incorrects + incorrect)
            },
        );

    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
