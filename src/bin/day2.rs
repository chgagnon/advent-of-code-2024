use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
}

fn is_report_safe(report: Vec<i32>) -> bool {
    let mut previous_level = report[0];
    let mut dir: Option<Direction> = None;
    for level in &report[1..] {
        let distance = (previous_level - level).abs();
        if distance < 1 || distance > 3 {
            return false;
        }

        match dir {
            Some(d) => {
                if *level > previous_level && d == Direction::Decreasing {
                    return false;
                }
                if *level < previous_level && d == Direction::Increasing {
                    return false;
                }
            }
            None => {
                if *level > previous_level {
                    dir = Some(Direction::Increasing);
                } else {
                    dir = Some(Direction::Decreasing);
                }
            }
        }
        previous_level = *level;
    }
    return true;
}

fn get_part_1() -> u32 {
    let file_path = "inputs/day2.txt";
    let file = File::open(file_path).expect("File read failed");
    let reader = BufReader::new(file).lines();

    let count = reader.fold(0, |acc, line| {
        acc + is_report_safe(
            line.unwrap()
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        ) as u32
    });
    count
}
fn main() {
    let num_safe = get_part_1();
    println!("There are {} safe reports", num_safe);
}
