use std::collections::{BinaryHeap, HashSet};

use aoc2024_rust::read_from_args;

#[derive(Debug)]
struct Tile {
    score: i32,
    coords: (usize, usize),
    direction: usize, // 0 N, 1 E, 2 S, 3 W
    prev_path: Vec<(usize, usize)>,
}

impl Eq for Tile {}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_new_tiles(cur_tile: &Tile) -> [Tile; 3] {
    let clockwise = Tile {
        score: cur_tile.score + 1000,
        coords: cur_tile.coords,
        direction: match cur_tile.direction {
            0 => 1,
            1 => 2,
            2 => 3,
            _ => 0,
        },
        prev_path: cur_tile.prev_path.clone(),
    };

    let counter_clockwise = Tile {
        score: cur_tile.score + 1000,
        coords: cur_tile.coords,
        direction: match cur_tile.direction {
            0 => 3,
            1 => 0,
            2 => 1,
            _ => 2,
        },
        prev_path: cur_tile.prev_path.clone(),
    };

    let mut new_prev_path = cur_tile.prev_path.clone();
    new_prev_path.push(cur_tile.coords);

    let same_dir = Tile {
        score: cur_tile.score + 1,
        coords: match cur_tile.direction {
            0 => (cur_tile.coords.0 - 1, cur_tile.coords.1),
            1 => (cur_tile.coords.0, cur_tile.coords.1 + 1),
            2 => (cur_tile.coords.0 + 1, cur_tile.coords.1),
            _ => (cur_tile.coords.0, cur_tile.coords.1 - 1),
        },
        direction: cur_tile.direction,
        prev_path: new_prev_path,
    };

    [same_dir, clockwise, counter_clockwise]
}

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start_coords = maze
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|ch| ch == &'S').map(|col| (row, col)))
        .unwrap();

    let end_coords = maze
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|ch| ch == &'E').map(|col| (row, col)))
        .unwrap();

    let mut paths: Vec<(Vec<(usize, usize)>, i32)> = vec![];
    let mut seen: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut min_heap: BinaryHeap<Tile> = BinaryHeap::new();

    min_heap.push(Tile {
        score: 0,
        coords: start_coords,
        direction: 1,
        prev_path: vec![],
    });

    while !min_heap.is_empty() {
        let cur_dir = min_heap.peek().unwrap().direction;
        let (cur_row, cur_col) = min_heap.peek().unwrap().coords;
        let cur_score = min_heap.peek().unwrap().score;

        let mut to_add_to_seen = vec![(cur_row, cur_col, cur_dir)];
        while min_heap.peek().is_some() && min_heap.peek().unwrap().score == cur_score {
            let cur_tile = min_heap.pop().unwrap();

            let (cur_row, cur_col) = cur_tile.coords;

            if seen.contains(&(cur_row, cur_col, cur_tile.direction))
                || maze[cur_row][cur_col] == '#'
            {
                continue;
            }

            to_add_to_seen.push((cur_row, cur_col, cur_tile.direction));

            if cur_tile.coords == end_coords {
                if paths.is_empty() || paths[0].1 == cur_tile.score {
                    paths.push((cur_tile.prev_path, cur_tile.score));
                }

                continue;
            }

            get_new_tiles(&cur_tile)
                .into_iter()
                .for_each(|tile| min_heap.push(tile));
        }

        to_add_to_seen.iter().for_each(|tile_data| {
            seen.insert(*tile_data);
        })
    }

    let part_one = paths[0].1;
    let parts_of_path: HashSet<(usize, usize)> =
        HashSet::from_iter(paths.into_iter().flat_map(|p| p.0));
    let part_two = parts_of_path.len() + 1;

    println!("part one: {part_one}");
    println!("part two: {part_two}")
}
