use std::collections::{BinaryHeap, HashMap};

use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    // using a binary heap just for fun
    let lists: [Vec<i32>; 2] = input
        .lines()
        .fold([BinaryHeap::new(), BinaryHeap::new()], |mut lists, line| {
            let values: Vec<i32> = line
                .split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect();
            lists[0].push(values[0]);
            lists[1].push(values[1]);

            lists
        })
        .map(|list| list.into_sorted_vec());

    let part_one = lists[0].iter().enumerate().fold(0, |distance, (i, curr)| {
        distance + (curr - lists[1][i]).abs()
    });

    let appearances = lists[1].iter().fold(HashMap::new(), |mut apps, curr| {
        apps.entry(curr).and_modify(|app| *app += 1).or_insert(1);
        apps
    });

    let part_two = lists[0].iter().fold(0, |similarity, curr| {
        similarity + (curr * appearances.get(curr).unwrap_or(&0))
    });

    println!("{part_one}");
    println!("{part_two}");
}
