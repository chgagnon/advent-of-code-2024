use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
}

fn distance_out_of_range(previous: i32, current: i32) -> bool {
    let distance = (previous - current).abs();
    distance < 1 || distance > 3
}

fn set_or_check_dir(dir: &mut Option<Direction>, previous: i32, current: i32) -> bool {
    match dir {
        Some(Direction::Increasing) => {
            return current >= previous;
        }
        Some(Direction::Decreasing) => {
            return current <= previous;
        }
        None => {
            if current > previous {
                *dir = Some(Direction::Increasing);
            } else {
                *dir = Some(Direction::Decreasing);
            }
            return true;
        }
    }
}

fn is_report_safe_1(report: Vec<i32>) -> bool {
    let mut previous_level = report[0];
    let mut dir: Option<Direction> = None;
    for level in &report[1..] {
        if distance_out_of_range(previous_level, *level) {
            return false;
        }
        if !set_or_check_dir(&mut dir, previous_level, *level) {
            return false;
        }
        previous_level = *level;
    }
    return true;
}

fn is_report_safe_2(report: Vec<i32>) -> bool {
    // If the first level can be removed and the rest of the report is safe,
    // then the report is safe under the definition for part 2.
    if is_report_safe_1(report[1..].to_vec()) {
        return true;
    }
    println!("Was NOT safe after dropping first elem:");
    dbg!(&report);
    // Since the report is not safe when the first level is removed, we can
    // assume that, even if the report must have a level removed, it will
    // not be the first level.
    let mut previous_level = report[0];
    let mut dir: Option<Direction> = None;
    let mut skipped_level_already = false;

    for level in &report[1..] {
        if distance_out_of_range(previous_level, *level) {
            if skipped_level_already {
                return false;
            } else {
                skipped_level_already = true;
                continue;
            }
        }

        if !set_or_check_dir(&mut dir, previous_level, *level) {
            if skipped_level_already {
                return false;
            } else {
                skipped_level_already = true;
                continue;
            }
        }
        previous_level = *level;
    }
    return true;
}

fn get_num_safe(is_report_safe: fn(Vec<i32>) -> bool) -> u32 {
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
    let num_safe = get_num_safe(is_report_safe_2);
    println!("There are {} safe reports", num_safe);
}

#[cfg(test)]
mod tests {

    macro_rules! check_safety {
        ($vector:expr, $expected_result:expr) => {
            {
                let actual_result = is_report_safe_2($vector);
                assert_eq!(actual_result, $expected_result);
            }
        };
    }

    use super::*;

    #[test]
    fn big_test_suite() {
        check_safety!(vec![0, 1, 2, 3, 4, 5], true);
        check_safety!(vec![1, 1, 1, 1, 1, 1], false);
        check_safety!(vec![1, 1, 1, 10, 1, 1], false);
        check_safety!(vec![0, 1, 25, 3, 4, 5], true);
        check_safety!(vec![100, 1, 2, 3, 4, 5], true);
        check_safety!(vec![66, 67, 68, 71, 72, 69], true);
    }

}
