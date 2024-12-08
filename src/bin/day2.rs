use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
}

fn is_report_safe_1(report: Vec<i32>) -> bool {
    
}

fn is_report_safe_2(report: Vec<i32>) -> bool {
    let mut previous_level = -1;
    let mut dir: Option<Direction> = None;
    let mut skipped_level_already = false;
    for level in &report {
        let distance = (previous_level - level).abs();
        if distance < 1 || distance > 3 {
            if skipped_level_already {
                return false;
            } else {
                skipped_level_already = true;
                continue;
            }
        }

        match dir {
            Some(Direction::Increasing) => {
                if *level < previous_level {
                    if skipped_level_already {
                        return false;
                    } else {
                        skipped_level_already = true;
                        continue;
                    }
                }
            }
            Some(Direction::Decreasing) => {
                if *level > previous_level {
                    if skipped_level_already {
                        return false;
                    } else {
                        skipped_level_already = true;
                        continue;
                    }
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

fn get_num_safe() -> u32 {
    let file_path = "inputs/day2.txt";
    let file = File::open(file_path).expect("File read failed");
    let reader = BufReader::new(file).lines();

    let predicate = if use_part_2 {is_report_safe_2} else is_report_safe_1;

    let count = reader.fold(0, |acc, line| {
        acc + is_report_safe_2(
            line.unwrap()
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        ) as u32
    });
    count
}
fn main() {
    let num_safe = get_num_safe();
    println!("There are {} safe reports", num_safe);
}
