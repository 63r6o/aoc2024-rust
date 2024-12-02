use aoc2024_rust::read_from_args;

fn main() {
    let input = read_from_args().expect("Couldn't open the input file");
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|level| level.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    fn is_safe(report: &Vec<i32>) -> Option<&Vec<i32>> {
        let increasing = report[0] < report[1];
        for (i, level) in report.iter().enumerate().skip(1) {
            let diff = (report[i - 1] - level).abs();
            if !(1..=3).contains(&diff) {
                return None;
            }

            match (increasing, &report[i - 1] < level) {
                (true, true) => continue,
                (true, false) => return None,
                (false, true) => return None,
                (false, false) => continue,
            }
        }
        Some(report)
    }

    let part_one = reports
        .iter()
        .filter_map(|report| is_safe(report))
        .collect::<Vec<&Vec<i32>>>()
        .len();

    let part_two = reports
        .iter()
        .filter_map(|report| {
            if is_safe(report).is_some() {
                return Some(report);
            }

            for i in 0..report.len() {
                let mut cloned_report = report.clone();
                cloned_report.remove(i);

                if is_safe(&cloned_report).is_some() {
                    return Some(report);
                }
            }
            None
        })
        .collect::<Vec<&Vec<i32>>>()
        .len();

    println!("part one: {part_one}");
    println!("part two: {part_two}");
}
