use std::fs::File;
use std::io::{BufReader, BufRead};

fn is_report_safe(report: Vec<i32>) -> bool {

}

fn get_part_1(grid: &Vec<Vec<i32>>) -> u32 {
    let file_path = "inputs/day2.txt";
    let file = File::open(file_path).expect("File read failed");
    let reader = BufReader::new(file).lines();
    
    let count = reader.fold(0, |acc, line| acc + is_report_safe(line.unwrap().split(" ").map(|s| s.parse::<i32>().unwrap()).collect()));

    let contents: Vec<Vec<i32>> = fs::read_to_string(file_path)
        .expect("File read failed")
        .lines()
        .map(|s| s.split("   ").map(|n| n.parse::<i32>().unwrap()).collect())

}
fn main() {
    let num_safe = get_part_1(&contents);
    println!("There are {} safe reports", num_safe);
}
