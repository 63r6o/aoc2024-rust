use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");

    let robots: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').map(|(pos, vel)| {
                let positions = pos.split_once('=').map(|(_, p)| {
                    p.split_once(',')
                        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                });

                let velocity = vel.split_once('=').map(|(_, v)| {
                    v.split_once(',')
                        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                });

                (positions.flatten().unwrap(), velocity.flatten().unwrap())
            })
        })
        .collect();

    let width = 101;
    let height = 103;
    let steps = 100;

    let step = |pos: &(i32, i32), vel: &(i32, i32), number_of_steps: i32| {
        let x_offset = (pos.0 + (vel.0 * number_of_steps)) % width;
        let y_offset = (pos.1 + (vel.1 * number_of_steps)) % height;

        let new_x = if x_offset < 0 {
            width + x_offset
        } else {
            x_offset
        };

        let new_y = if y_offset < 0 {
            height + y_offset
        } else {
            y_offset
        };

        (new_x, new_y)
    };

    let part_one: i32 = robots
        .iter()
        .map(|(pos, vel)| step(pos, vel, steps))
        .fold([0, 0, 0, 0], |mut quadrants, (x, y)| {
            match (x.cmp(&(width / 2)), y.cmp(&(height / 2))) {
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quadrants[2] += 1,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quadrants[3] += 1,
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quadrants[0] += 1,
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quadrants[1] += 1,
                _ => (),
            }

            quadrants
        })
        .iter()
        .product();

    let mut part_two = 0;
    loop {
        let current_state: Vec<(i32, i32)> = robots
            .iter()
            .map(|(pos, vel)| step(pos, vel, part_two))
            .collect();

        if current_state
            .iter()
            .filter(|pos| {
                pos.0 > 1
                    && pos.0 < width - 1
                    && pos.1 < height - 1
                    && current_state.contains(&(pos.0 - 1, pos.1 + 1))
                    && current_state.contains(&(pos.0 + 1, pos.1 + 1))
            })
            .collect::<Vec<&(i32, i32)>>()
            .len()
            > current_state.len() / 2 / 3
        {
            let mut pic = [[' '; 101]; 103];
            current_state.iter().for_each(|(x, y)| {
                pic[*y as usize][*x as usize] = 'x';
            });

            pic.iter()
                .for_each(|line| println!("{}", line.iter().collect::<String>()));

            break;
        }
        part_two += 1;
    }

    println!("part one: {part_one}");
    println!("part two: {part_two}");
}
